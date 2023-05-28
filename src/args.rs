use std::{path::PathBuf, process::exit};

use anyhow::{anyhow, Result};
use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, PartialEq, ValueEnum)]
pub enum DumpType {
    Char,
    Line,
}

#[derive(Clone, Debug, Parser, Default)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Morse code speed in `wpm` units
    #[arg(short, long, default_value = "25")]
    pub wpm: u8,

    /// Morse code frequency
    #[arg(short, long, default_value = "600.0")]
    pub frequency: f32,

    /// Morse code volume
    #[arg(short, long, default_value = "0.2")]
    pub volume: f32,

    /// power for audio volume
    #[arg(long, default_value = "2.5")]
    pub power: f32,

    /// Dump message line by per char or per line
    #[arg(short, long)]
    pub dump: Option<DumpType>,

    /// Perform command analysis only
    #[arg(long)]
    debug: bool,

    // group 設定では、グループ内の項目は、すべて指定ないか、たかだか一つの項目を指定することはができる。
    // group の一つに必須条件をつければ、グループの一つの指定が必須となる
    /// The message directly as a command line argument
    #[arg(name = "TEXT", group("text"), required = true)]
    pub text: Option<String>,

    /// Read messages from standard input
    #[arg(short, long, group("text"))]
    pub pipe: bool,

    /// Read message from file
    #[arg(short, long, value_name = "FILE", group("text"))]
    pub input: Option<PathBuf>,
}

/// コマンドライン引数を解析し構造体に取り込む
/// オプションの範囲検査やファイルの有無の検査も行う
pub fn get_args() -> Result<Args> {
    let opt = Args::parse();
    println!("{:#?}", opt);

    // エラーならメッセージを出力して終了
    check_range(&opt)?;

    // デバッグモードならここで正常終了
    if opt.debug {
        exit(0)
    }
    return Ok(opt);
}

pub fn check_range(opt: &Args) -> Result<()> {
    if opt.wpm < 3 || 60 < opt.wpm {
        return Err(anyhow!("error: wpm out is of range ( 3 .. 60 )"));
    }

    if opt.frequency < 400.0 || 1200.0 < opt.frequency {
        return Err(anyhow!(
            "error: frequency is out of range ( 400.0 .. 1200.0 )"
        ));
    }

    if opt.volume < 0.001 || 1.0 < opt.volume {
        return Err(anyhow!("error: volume is out of range ( 0.001 .. 1.0 )"));
    }

    if opt.power < 1.0 || 5.0 < opt.power {
        return Err(anyhow!("error: power is out of range ( 1.0 .. 5.0 )"));
    }

    if let Some(path) = &opt.input {
        if let Ok(is_exist) = path.try_exists() {
            if !is_exist {
                return Err(anyhow!("error: file does not exist."));
            }
        } else {
            return Err(anyhow!("error: file is unavailable."));
        }
    }

    return Ok(());
}
