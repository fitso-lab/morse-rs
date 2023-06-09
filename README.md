It is written in <span style="color: red; ">Japanese</span>.<br>

# morse-rs
A CLI-based tool that reads text and generates Morse sounds.<br>
テキストを読み込み、モールス音を発生させるCLIベースのツール

# Usage
There are 3 types of startup methods for reading text.<br>
テキストの読み込み方で3種類の起動方法がある

```
Usage: morse-rs.exe [OPTIONS] [<TEXT>]

Arguments:
    <TEXT>  The message directly as a command line argument

Options:
  -w, --wpm <WPM>              Morse code speed in `wpm` units [default: 25]
  -f, --frequency <FREQUENCY>  Morse code frequency [default: 600.0]
  -v, --volume <VOLUME>        Morse code volume [default: 0.2]
      --power <POWER>          power for audio volume [default: 2.5]
      --farnsworth-timing <FARNSWORTH_TIMING>
                               Farnsworth timing [default: 1.0]
  -d, --dump <DUMP>            Dump message line by per char or per line [possible values: char, line]
      --debug                  Perform command analysis only
      --verbose                Verbose mode
  -p, --pipe                   Read messages from standard input
  -i, --input <FILE>           Read message from file
  -h, --help                   Print help
  -V, --version                Print version

ex.
  morse-rs "cq cq cq"
  morse-rs --pipe < hoge.txt
  morse-rs --input hoge.txt
```

# 和文符号と記号符号の一部について
  LCWO.netの「テキストをCWに変換」ページから辿った、[jscwlib](https://git.fkurz.net/dj1yfk/jscwlib/) から
  対応する符号をコピーしました。<br>
  **DJ1YFK** 様並びに **DJ5CW** 様に感謝です。

  JARLとARRLでも異なっていて<br>
  ```%``` を ```<0/0>``` としているサイトがあったりと、ローカルルールがあるのかもしれません。<br>
  記号符号の一部は、定義元が不明で通用するのかどうか不明です。<br>
  JARLとARRL共通は、```.,?-``` のようです。


# About text
  英文と和文。英字については大文字小文字問わない。和文はカタカナ・ひらがな問わない<br>
  ```<>```で囲んだ複数の文字は、文字間の短点3つ分の待ち無しに一文字のように出力する。
   
  - `#` 以降、行末までは、注釈として無視します。
  - 行頭が `#!` の行は、オプション指定[^1]行と解釈します。
    - オプションは、
      - `--wpm <WPM>`
      - `--frequency <FREQUENCY>`
      - `--volume <VOLUME>`
      - `--farnsworth-timing <FARNSWORTH_TIMING>`
      - `--player <PLAYER>`
    - `--player` で、名前をつけておくと、`--player` で名前を指定するだけで、以前の定義を利用できます。
[^1]: コマンドラインオプションと同じに見えますが、パーサーが違うため、全く同じ表記が通るとは限りません。

# Install

# Improvement
  - farnsworth-timingを導入。文字の速度はそのままに、文字や語間の速度を遅くできる。
  - 電文毎に音の高さや速度等の変更が可能<br>
    相手先毎に音の高さや速度等を変えることで区別して聞き取りやすくできる<br>
  - 音量を指数関数で変化するようにした<br>
    音量(0-1)をそのまま使用すると、いきなり大きくなり、後半は殆ど変化しない感じとなるため<br>
    オーディオボリュームのＣタイプの抵抗曲線となるように変換式を加えた。<br>
    自身の聴感では、2.5がちょうどよいのでデフォルトとした。<br>
  - 実行中に対応する行または文字を出力することで、どの音が出ているのかわかるようにした。<br>
  - コマンドラインの引数解析を `clap` で実現した。<br>
    3種類の電文の与え方についての、排他チェックを `clap` で行うことができた。<br>
    ただし、項目の範囲チェックやファイルの存在チェックは、別途実装した。

# License
MIT License