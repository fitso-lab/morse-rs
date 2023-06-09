use std::{
    fs::File,
    io::{stdin, BufReader},
};

use anyhow::Result;
use morse_rs::{args::get_args, morse::Morse};
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

fn main() -> Result<()> {
    let opt = get_args()?;

    let mut morse = Morse::new(&opt);

    if let Some(ref text) = opt.text {
        // コマンドラインに電文を記述
        let reader = StringReader::new(text);
        let mut bufreader = BufReader::new(reader);
        morse.play(&mut bufreader)?;
    } else if let Some(ref input) = opt.input {
        // 電文ファイルを指定
        let mut reader = BufReader::new(File::open(input.to_str().unwrap()).unwrap());
        morse.play(&mut reader)?;
    } else {
        // 標準入力から電文を取得
        let mut reader = BufReader::new(stdin());
        morse.play(&mut reader)?;
    }

    return Ok(());
}
