use std::{
    collections::HashMap,
    f32::consts::PI,
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    OutputCallbackInfo, Sample, Stream, StreamConfig,
};

use crate::{
    args::{Args, DumpType},
    translation_table::set_translation_table,
};

pub fn genarate_stream(frequency: f32, volume: f32, power: f32) -> Stream {
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
    println!("Output device: {}", device.name().unwrap());

    let config: StreamConfig = device.default_output_config().unwrap().into();
    println!("Default output config: {:?}", config);

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

#[derive(Clone)]
pub struct Morse {
    table: HashMap<char, &'static str>,

    dump: Option<DumpType>,
    dit_duration: u32,
}

impl Morse {
    pub fn new<'a>(opt: &'a Args) -> Morse {
        let table = set_translation_table();
        let dit_duration = 60 * 1000 / (50 * opt.wpm as u32);
        // streams.insert("default", &stream);

        return Morse {
            dump: opt.dump.clone(),
            dit_duration,
            table,
        };
    }

    pub fn play(&self, text: &str, stream: &Stream) {
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
                        self.litter_space();
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

    fn intra_space(&self) {
        let duration = Duration::from_millis(1 * self.dit_duration as u64);

        sleep(duration);
    }

    fn litter_space(&self) {
        let duration = Duration::from_millis(3 * self.dit_duration as u64);

        sleep(duration);
    }

    pub fn word_space(&self) {
        let duration = Duration::from_millis(7 * self.dit_duration as u64);

        sleep(duration);
    }
}
