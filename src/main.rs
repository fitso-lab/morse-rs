use std::{
    collections::HashMap,
    fs::File,
    io::{stdin, BufRead, BufReader},
    str::Split,
};

use anyhow::{anyhow, Result};
use morse_rs::{
    args::{check_range, get_args, Args},
    morse::{calc_dit, genarate_stream, Morse},
};
use stringreader::StringReader;

/// Intra-character space <br>
/// Inter-character space

/// ```
/// 10 WPM = 50 CPM(=PARIS方式の通信速度)
/// PARIS = 50短点
///   内訳
///     "PARIS" -> ".--. .- .-. .. ..."
///      -> 1*Σ.=10
///       + 3*Σ-=3*4=12
///       + 1*Σ{文字内ギャップ}=10+4-5=9
///       + 3*Σ{文字間ギャップ}=3*4=12
///       + 7*Σ{単語間ギャップ}=7*1=7
///      -> 10+12+9+12+7=50//
/// 短点[ms]  = 60 * 1000 / (50 * wpm);

/// (JARLの「モールス電信技能認定」)[https://www.jarl.org/Japanese/1_Tanoshimo/1-4_Morse/Morse.htm]での速度
/// | 段位 | CPM| WPM | 短点[ms] | 長点[ms] |
/// | 3級  | 25 |   5 | 240.00  | 720.00   |第３級アマチュア無線技士相当|
/// | 2級  | 45 |   9 | 133.33  | 400.00   |第２級アマチュア無線技士相当|
/// | 1級  | 60 |  12 | 100.00  | 300.00   |第１級アマチュア無線技士相当|
/// | 初段 | 90 |  18 |  66.67  | 200.00   |国内電信級陸上特殊無線技士相当|
/// | 2段  |110 |  22 |  54.55  | 163.64   |第２・３級総合無線通信士相当|
/// | 3段  |120 |  24 |  50.00  | 150.00   |第１級総合無線通信士相当|
/// | 4段  |140 |  28 |  42.86  | 128.57   |
/// | 5段  |160 |  32 |  37.50  | 112.50   |
/// |名人位|180 |  36 |  33.33  | 100.00   |
/// モールス通信 和文 CPMの７-８割
///             欧文普通語 CPMとほぼ同じ
///             欧文暗語 CPMの８-９割
/// 600Hz ... 55.555 回 per 33.33ms
/// ```

fn play<R>(reader: &mut R, morse: &mut Morse) -> Result<()>
where
    R: BufRead,
{
    let mut is_first = true;

    let mut frequency = morse.frequency;
    let mut volume = morse.volume;
    let mut wpm = morse.wpm;
    let mut farnsworth_timing = morse.farnsworth_timing;
    // let mut dit_duration = morse.dit_duration;

    // let mut player: Option<&str> = None;
    let mut stream = genarate_stream(frequency, volume, morse.power);

    let mut players: HashMap<String, (f32, f32, u8, f32, u32)> = HashMap::from([(
        "default".to_string(),
        (
            frequency,
            volume,
            wpm,
            farnsworth_timing,
            morse.dit_duration,
        ),
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
                if let Some(w) = o_frequency {
                    frequency = w;
                }
                if let Some(w) = o_volume {
                    volume = w;
                }
                if let Some(w) = o_wpm {
                    wpm = w;
                }
                if let Some(w) = o_farnsworth_timing {
                    farnsworth_timing = w;
                }
                let mut opt = Args::default();
                opt.wpm = wpm;
                opt.farnsworth_timing = farnsworth_timing;
                opt.frequency = frequency;
                opt.volume = volume;
                opt.power = morse.power;

                //TODO 範囲チェックエラーは、メッセージを出力して終了。暫定
                check_range(&opt)?;
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
                            morse.dit_duration = *w_dit_duration;
                        } else {
                            return Err(anyhow!("Warning: `player`({}) is not defined.", w));
                        }
                    } else {
                        // どれか一つでも指定されているのなら、'player'定義として登録
                        morse.dit_duration = calc_dit(wpm);
                        players.insert(
                            w.to_string(),
                            (
                                frequency,
                                volume,
                                wpm,
                                farnsworth_timing,
                                morse.dit_duration,
                            ),
                        );
                    }
                }

                stream = genarate_stream(frequency, volume, morse.power);

                println!(
                    "Option({}): frequency({:#?}) volume({:#?}) wpm({:#?}) farnsworth_timing({:#?}) player({:#?})",
                    line, o_frequency, o_volume, o_wpm, o_farnsworth_timing, o_player
                );
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
            morse.word_space();
        }

        morse.play(&line, &stream);
    }

    return Ok(());
}

fn main() -> Result<()> {
    let opt = get_args()?;

    let mut morse = Morse::new(&opt);

    if let Some(ref text) = opt.text {
        // コマンドラインに電文を記述
        let reader = StringReader::new(text);
        let mut bufreader = BufReader::new(reader);
        play(&mut bufreader, &mut morse)?;
    } else if let Some(ref input) = opt.input {
        // 電文ファイルを指定
        let mut reader = BufReader::new(File::open(input.to_str().unwrap()).unwrap());
        play(&mut reader, &mut morse)?;
    } else {
        // 標準入力から電文を取得
        let mut reader = BufReader::new(stdin());
        play(&mut reader, &mut morse)?;
    }

    return Ok(());
}
