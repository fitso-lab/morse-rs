use std::{
    collections::HashMap,
    f32::consts::PI,
    io::{stdout, BufRead, Write},
    str::Split,
    thread::sleep,
    time::Duration,
};

use anyhow::{anyhow, Result};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    OutputCallbackInfo, Sample, Stream, StreamConfig,
};

use crate::{
    args::{check_range, Args, DumpType},
    translation_table::set_translation_table,
};

/// 周波数と音量を指定して発音用の stream を生成する
fn genarate_stream(frequency: f32, volume: f32, power: f32) -> Stream {
    fn write_data(output: &mut [f32], channels: usize, next_sample: &mut dyn FnMut() -> f32) {
        for frame in output.chunks_mut(channels) {
            let value: f32 = f32::from_sample(next_sample());
            for sample in frame.iter_mut() {
                *sample = value;
            }
        }
    }

    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("failed to find output device");

    let config: StreamConfig = device.default_output_config().unwrap().into();

    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    // 数値そのままだと、音量の変化が不自然(小音で急激に、大音で微小)なので、オーディオ用のＣカーブ特性のボリュームを再現。デフォルトの数値は感覚で決定
    let vol = volume.powf(power);

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0.0;
    let frequency = frequency;
    let mut next_value = move || {
        let ret = (sample_clock * frequency * 2.0 * PI / sample_rate).sin() * vol;

        sample_clock = sample_clock + 1.0;

        return ret;
    };

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &OutputCallbackInfo| {
                write_data(data, channels, &mut next_value)
            },
            err_fn,
            None,
        )
        .unwrap();

    return stream;
}

/// wpm から短点の長さを算出する
fn calc_dit(wpm: u8) -> u32 {
    return 60 * 1000 / (50 * wpm as u32);
}

/// オプション内容を出力
fn println_option(
    o_player: Option<String>,
    o_frequency: Option<f32>,
    o_volume: Option<f32>,
    o_wpm: Option<u8>,
    o_farnsworth_timing: Option<f32>,
) {
    print!("#!");

    if let Some(player) = o_player {
        print!(" --player {}", player);
    }
    if let Some(frequency) = o_frequency {
        print!(" --frequency {}", frequency);
    }
    if let Some(volume) = o_volume {
        print!(" --volume {}", volume);
    }
    if let Some(wpm) = o_wpm {
        print!(" --wpm {}", wpm);
    }
    if let Some(farnsworth_timing) = o_farnsworth_timing {
        print!(" --farnsworth_timing {}", farnsworth_timing);
    }

    println!();
}

#[derive(Clone)]
pub struct Morse {
    /// 文字->モールス音変換用テーブル
    table: HashMap<char, &'static str>,

    /// 出力単位
    dump: Option<DumpType>,

    /// オプションの詳細
    verbose: bool,

    /// 短点の長さ
    dit_duration: u32,
    /// 周波数
    frequency: f32,
    /// 音量
    volume: f32,
    /// wpm
    wpm: u8,
    /// 文字・語間の長さ倍率
    farnsworth_timing: f32,
    /// 音量換算値
    power: f32,
}

impl Morse {
    pub fn new<'a>(opt: &'a Args) -> Morse {
        let table = set_translation_table();
        let dit_duration = calc_dit(opt.wpm);
        // streams.insert("default", &stream);

        return Morse {
            table,
            dump: opt.dump.clone(),
            verbose: opt.verbose,
            dit_duration,
            frequency: opt.frequency,
            volume: opt.volume,
            wpm: opt.wpm,
            farnsworth_timing: opt.farnsworth_timing,
            power: opt.power,
        };
    }

    /// テキストをモールス符号に変換、発音する
    fn play_sound(&self, text: &str, stream: &Stream) {
        let mut is_first = true;
        let mut is_one = false;

        if let Some(dump) = &self.dump {
            if *dump == DumpType::Line {
                println!("{}", text);
            }
        }

        for ch in text.chars() {
            if !is_first {
                if !is_one {
                    self.litter_space();
                }
            } else {
                is_first = false;
            }
            if let Some(m) = self.table.get(&ch) {
                if let Some(dump) = &self.dump {
                    if *dump == DumpType::Char {
                        print!("{}", ch);
                        stdout().flush().unwrap();
                    }
                }
                m.chars().for_each(|c| match c {
                    '.' => {
                        stream.play().unwrap();
                        self.intra_space();
                        stream.pause().unwrap();

                        self.intra_space();
                    }
                    '-' => {
                        stream.play().unwrap();
                        self.dash_space();
                        stream.pause().unwrap();
                        self.intra_space();
                    }
                    ' ' => {
                        self.word_space();
                    }
                    '<' => {
                        is_one = true;
                    }
                    '>' => {
                        is_one = false;
                    }
                    _ => {}
                });
            }
        }
        if let Some(dump) = &self.dump {
            if *dump == DumpType::Char {
                println!();
            }
        }
    }

    /// 文字内の短点の時間
    fn intra_space(&self) {
        let duration = Duration::from_millis(1 * self.dit_duration as u64);

        sleep(duration);
    }

    /// 長点の時間
    fn dash_space(&self) {
        let duration = Duration::from_millis(3 * self.dit_duration as u64);

        sleep(duration);
    }

    /// 文字間の時間
    fn litter_space(&self) {
        let duration =
            Duration::from_millis((3.0 * self.dit_duration as f32 * self.farnsworth_timing) as u64);

        sleep(duration);
    }

    /// 語間の時間
    fn word_space(&self) {
        let duration =
            Duration::from_millis((7.0 * self.dit_duration as f32 * self.farnsworth_timing) as u64);

        sleep(duration);
    }

    /// コマンドライン・オプションの範囲チェック機能を利用してオプションの範囲チェックを実施
    fn option_check(
        &self,
        o_frequency: Option<f32>,
        o_volume: Option<f32>,
        o_wpm: Option<u8>,
        o_farnsworth_timing: Option<f32>,
        frequency: &mut f32,
        volume: &mut f32,
        wpm: &mut u8,
        farnsworth_timing: &mut f32,
    ) -> Result<()> {
        if let Some(w) = o_frequency {
            *frequency = w;
        }
        if let Some(w) = o_volume {
            *volume = w;
        }
        if let Some(w) = o_wpm {
            *wpm = w;
        }
        if let Some(w) = o_farnsworth_timing {
            *farnsworth_timing = w;
        }
        let mut opt = Args::default();
        opt.wpm = *wpm;
        opt.farnsworth_timing = *farnsworth_timing;
        opt.frequency = *frequency;
        opt.volume = *volume;
        opt.power = self.power;

        check_range(&opt)?;

        return Ok(());
    }
    /// 入力先の文字列（複数行）をモールス発音
    pub fn play<R>(&mut self, reader: &mut R) -> Result<()>
    where
        R: BufRead,
    {
        let mut is_first = true;

        let mut frequency = self.frequency;
        let mut volume = self.volume;
        let mut wpm = self.wpm;
        let mut farnsworth_timing = self.farnsworth_timing;
        // let mut dit_duration = morse.dit_duration;

        // let mut player: Option<&str> = None;
        let mut stream = genarate_stream(frequency, volume, self.power);

        let mut players: HashMap<String, (f32, f32, u8, f32, u32)> = HashMap::from([(
            "default".to_string(),
            (frequency, volume, wpm, farnsworth_timing, self.dit_duration),
        )]);

        for (_, result) in reader.lines().enumerate() {
            let mut line = result.unwrap();
            if line.starts_with("#!") {
                // 行頭がオプション定義ならオプションとして解釈
                let mut split: Split<char> = line.split('!');
                split.next();
                if let Some(l) = split.next() {
                    if l.len() == 0 {
                        continue;
                    }
                    let mut s = l.split_whitespace();

                    let mut o_frequency: Option<f32> = None;
                    let mut o_volume: Option<f32> = None;
                    let mut o_wpm: Option<u8> = None;
                    let mut o_farnsworth_timing: Option<f32> = None;
                    let mut o_player: Option<String> = None;

                    loop {
                        if let Some(w) = s.next() {
                            match w.to_lowercase().as_str() {
                                "--frequency" => {
                                    if let Some(v) = s.next() {
                                        if o_frequency.is_none() {
                                            // 数値を取り込む
                                            o_frequency = Some(v.parse().unwrap());
                                        } else {
                                            return Err(anyhow!(
                                                "Warning: Multiple 'frequency' are defined."
                                            ));
                                        }
                                    }
                                }
                                "--volume" => {
                                    if let Some(v) = s.next() {
                                        if o_volume.is_none() {
                                            // 数値を取り込む
                                            o_volume = Some(v.parse().unwrap());
                                        } else {
                                            return Err(anyhow!(
                                                "Warning: Multiple `volume` are defined."
                                            ));
                                        }
                                    }
                                }
                                "--wpm" => {
                                    if let Some(v) = s.next() {
                                        if o_wpm.is_none() {
                                            // 数値を取り込む
                                            o_wpm = Some(v.parse().unwrap());
                                        } else {
                                            return Err(anyhow!(
                                                "Warning: Multiple `wpm` are defined."
                                            ));
                                        }
                                    }
                                }
                                "--farnsworth_timing" => {
                                    if let Some(v) = s.next() {
                                        if o_farnsworth_timing.is_none() {
                                            // 数値を取り込む
                                            o_farnsworth_timing = Some(v.parse().unwrap());
                                        } else {
                                            return Err(anyhow!(
                                            "Warning: Multiple `farnsworth timing` are defined."
                                        ));
                                        }
                                    }
                                }
                                "--player" => {
                                    if let Some(v) = s.next() {
                                        if o_player.is_none() {
                                            // 重複チェック
                                            o_player = Some(v.trim().to_string());
                                        } else {
                                            return Err(anyhow!(
                                                "Warning: Multiple `player` are defined."
                                            ));
                                        }
                                    }
                                }
                                // 想定外のものは無視
                                _ => {
                                    return Err(anyhow!("Warning: Undefined word({}).", w));
                                }
                            }
                        } else {
                            // 取り出せなくなれば終了
                            break;
                        }
                    }
                    self.option_check(
                        o_frequency,
                        o_volume,
                        o_wpm,
                        o_farnsworth_timing,
                        &mut frequency,
                        &mut volume,
                        &mut wpm,
                        &mut farnsworth_timing,
                    )?;

                    if let Some(ref w) = o_player {
                        if o_frequency.is_none() && o_volume.is_none() && o_wpm.is_none() {
                            // すべて指定なしの場合は、定義されたプレイヤーを参照する
                            if let Some((
                                w_frequency,
                                w_volume,
                                w_wpm,
                                w_farnsworth_timing,
                                w_dit_duration,
                            )) = players.get(w)
                            {
                                frequency = *w_frequency;
                                volume = *w_volume;
                                wpm = *w_wpm;
                                farnsworth_timing = *w_farnsworth_timing;
                                self.dit_duration = *w_dit_duration;
                            } else {
                                return Err(anyhow!("Warning: `player`({}) is not defined.", w));
                            }
                        } else {
                            // どれか一つでも指定されているのなら、'player'定義として登録
                            self.dit_duration = calc_dit(wpm);
                            players.insert(
                                w.to_string(),
                                (frequency, volume, wpm, farnsworth_timing, self.dit_duration),
                            );
                        }
                    }

                    stream = genarate_stream(frequency, volume, self.power);

                    if self.verbose {
                        println_option(o_player, o_frequency, o_volume, o_wpm, o_farnsworth_timing);
                    }

                    continue;
                }
            } else {
                // '#'以降の文字列をコメントとして破棄
                if let Some(l) = line.split('#').next() {
                    line = l.trim().to_string();
                }
                if line.len() == 0 {
                    // 空行は無視
                    continue;
                }
            }
            if is_first {
                is_first = false;
            } else {
                self.word_space();
            }

            self.play_sound(&line, &stream);
        }

        return Ok(());
    }
}
