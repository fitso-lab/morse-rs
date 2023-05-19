# morse-rs
テキストを読み込み、モールス音を発生させるCLIベースのツール

# Usage
テキストの読み込み方で3種類の起動方法がある

Usage: morse-rs.exe [OPTIONS] [\<TEXT\>]

Arguments:
  \<TEXT\>  The message directly as a command line argument

Options:
  -w, --wpm <WPM>              Morse code speed in `wpm` units [default: 25] <br>
  -f, --frequency <FREQUENCY>  Morse code frequency [default: 600.0] <br>
  -v, --volume <VOLUME>        Morse code volume [default: 0.2] <br>
      --power <POWER>          power for audio volume [default: 2.5] <br>
  -d, --dump <DUMP>            Dump message line by per char or per line [possible values: char, line] <br>
      --debug                  Perform command analysis only <br>
  -p, --pipe                   Read messages from standard input <br>
  -i, --input <FILE>           Read message from file <br>
  -h, --help                   Print help <br>
  -V, --version                Print version <br>

ex. <br>
   ```morse-rs "cq cq cq"``` <br>
   ```morse-rs --pipe < hoge.txt``` <br>
   ```morse-rs --input hoge.txt"``` <br>

# 和文符号と記号符号の一部について
   LCWO.netの「テキストをCWに変換」ページから辿った、[jscwlib](https://git.fkurz.net/dj1yfk/jscwlib/)から
   対応する符号をコピーしました。<br>
   DJ1YFK様並びにDJ5CW様に感謝です。

   JARLとARRLでも異なっていて<br>
   ```%``` を ```<0/0>``` としているサイトがあったりと、ローカルルールがあるのかもしれません。<br>
   記号符号の一部は、定義元が不明で通用するのかどうか不明です。<br>
   JARLとARRL共通は、```.,?-``` のようです。


# About text
   英文と和文。英字については大文字小文字問わない。和文はカタカナ・ひらがな問わない<br>
   ```<>```で囲んだ複数の文字は、文字間の短点3つ分の待ち無しに一文字のように出力する。
   
# Install

# Improvement
   - 音量を指数関数で変化するようにした<br>
     音量(0-1)をそのまま使用すると、いきなり大きくなり、後半は殆ど変化しない感じとなるため<br>
     オーディオボリュームのＣタイプの抵抗曲線となるように変換式を加えた。<br>
     自身の聴感では、2.5がちょうどよいのでデフォルトとした。<br?
   - 実行中に対応する行または文字を出力することで、どの音が出ているのかわかるようにした。<br>
   - コマンドラインの引数解析を `clap` で実現した。<br>
     3種類の電文の与え方についての、排他チェックを `clap` で行うことができた。<br>
     ただし、項目の範囲チェックやファイルの存在チェックは、別途実装した。

# ToDo
- 複数の行で音の高さや速度等を変更する<br>
  複数行を出力できるようにした結果、通信のやり取りを出力できるようになった。双方で音の高さや速度を変更することで聞き取りやすくする<br>

# License
MIT License