use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATION_TABLE: Lazy<HashMap<char, &'static str>> = Lazy::new(|| {
    return set_translation_table();
});

/// 文字 -> モールスコード変換テーブルを作成する
fn set_translation_table() -> HashMap<char, &'static str> {
    let mut table: HashMap<char, &str> = HashMap::new();
    let tbl: Vec<(Vec<char>, &str)> = vec![
        (vec![' '], " "),
        //
        // 英文字
        (vec!['A', 'a'], ".-"),
        (vec!['B', 'b'], "-..."),
        (vec!['C', 'c'], "-.-."),
        (vec!['D', 'd'], "-.."),
        (vec!['E', 'e'], "."),
        (vec!['F', 'f'], "..-."),
        (vec!['G', 'g'], "--."),
        (vec!['H', 'h'], "...."),
        (vec!['I', 'i'], ".."),
        (vec!['J', 'j'], ".---"),
        (vec!['K', 'k'], "-.-"),
        (vec!['L', 'l'], ".-.."),
        (vec!['M', 'm'], "--"),
        (vec!['N', 'n'], "-."),
        (vec!['O', 'o'], "---"),
        (vec!['P', 'p'], ".--."),
        (vec!['Q', 'q'], "--.-"),
        (vec!['R', 'r'], ".-."),
        (vec!['S', 's'], "..."),
        (vec!['T', 't'], "-"),
        (vec!['U', 'u'], "..-"),
        (vec!['V', 'v'], "...-"),
        (vec!['W', 'w'], ".--"),
        (vec!['X', 'x'], "-..-"),
        (vec!['Y', 'y'], "-.--"),
        (vec!['Z', 'z'], "--.."),
        //
        // 数字
        (vec!['1'], ".----"),
        (vec!['2'], "..---"),
        (vec!['3'], "...--"),
        (vec!['4'], "....-"),
        (vec!['5'], "....."),
        (vec!['6'], "-...."),
        (vec!['7'], "--..."),
        (vec!['8'], "---.."),
        (vec!['9'], "----."),
        (vec!['0'], "-----"),
        //
        // 記号
        (vec!['.'], ".-.-.-"),
        (vec![','], "--..--"),
        (vec!['?'], "..--.."),
        (vec!['='], "-...-"),
        (vec!['-'], "-....-"),
        (vec![':'], "---..."),
        (vec!['\''], ".----."),
        //
        // jscwlib.jsから転記。重複削除及び注記追加
        (vec!['/'], "-..-."),
        (vec!['+'], ".-.-."),
        (vec!['('], "-.--."),
        (vec![')'], "-.--.-"),
        (vec!['@'], ".--.-."),
        (vec!['"'], ".-..-."),
        (vec!['!'], "..--."), /* same '?' */
        (vec!['$'], "...-..-"),
        (vec!['`'], ".-----."),
        (vec!['&'], ". ..."), /* same "ES" */
        (vec![';'], "-.-.-."),
        (vec!['«'], ".-..-."), /* same '"' */
        (vec!['»'], ".-..-."), /* same '"' */
        //
        // かな(清音) - いろは歌順＋かな文字混在なので厳密には清音ではない / KANA - a voiceless sound
        // jscwlib.jsから転記。注記一部変更
        (vec!['イ', 'い'], ".-"),    /* i  */
        (vec!['ロ', 'ろ'], ".-.-"),  /* ro */
        (vec!['ハ', 'は'], "-..."),  /* ha */
        (vec!['ニ', 'に'], "-.-."),  /* ni */
        (vec!['ホ', 'ほ'], "-.."),   /* ho */
        (vec!['ヘ', 'へ'], "."),     /* he */
        (vec!['ト', 'と'], "..-.."), /* to */
        (vec!['チ', 'ち'], "..-."),  /* ti */
        (vec!['リ', 'り'], "--."),   /* ri */
        (vec!['ヌ', 'ぬ'], "...."),  /* nu */
        (vec!['ル', 'る'], "-.--."), /* ru */
        (vec!['ヲ', 'を'], ".---"),  /* wo */
        (vec!['ワ', 'わ'], "-.-"),   /* wa */
        (vec!['カ', 'か'], ".-.."),  /* ka */
        (vec!['ヨ', 'よ'], "--"),    /* yo */
        (vec!['ョ', 'ょ'], "--"),    /* yo (small) */
        (vec!['タ', 'た'], "-."),    /* ta */
        (vec!['レ', 'れ'], "---"),   /* re */
        (vec!['ソ', 'そ'], "---."),  /* so */
        (vec!['ツ', 'つ'], ".--."),  /* tu */
        (vec!['ッ', 'っ'], ".--."),  /* tu (small) / a geminated consonant */
        (vec!['ネ', 'ね'], "--.-"),  /* ne */
        (vec!['ナ', 'な'], ".-."),   /* na */
        (vec!['ラ', 'ら'], "..."),   /* ra */
        (vec!['ム', 'む'], "-"),     /* mu */
        (vec!['ウ', 'う'], "..-"),   /* u  */
        (vec!['ヰ', 'ゐ'], ".-..-"), /* yi */
        (vec!['ノ', 'の'], "..--"),  /* no */
        (vec!['オ', 'お'], ".-..."), /* o  */
        (vec!['ク', 'く'], "...-"),  /* ku */
        (vec!['ヤ', 'や'], ".--"),   /* ya */
        (vec!['ャ', 'ゃ'], ".--"),   /* ya (small) */
        (vec!['マ', 'ま'], "-..-"),  /* ma */
        (vec!['ケ', 'け'], "-.--"),  /* ke */
        (vec!['フ', 'ふ'], "--.."),  /* fu */
        (vec!['コ', 'こ'], "----"),  /* ko */
        (vec!['エ', 'え'], "-.---"), /* e  */
        (vec!['テ', 'て'], ".-.--"), /* te */
        (vec!['ア', 'あ'], "--.--"), /* a  */
        (vec!['サ', 'さ'], "-.-.-"), /* sa */
        (vec!['キ', 'き'], "-.-.."), /* ki */
        (vec!['ユ', 'ゆ'], "-..--"), /* yu */
        (vec!['ュ', 'ゅ'], "-..--"), /* yu (small) */
        (vec!['メ', 'め'], "-...-"), /* me */
        (vec!['ミ', 'み'], "..-.-"), /* mi */
        (vec!['シ', 'し'], "--.-."), /* si */
        (vec!['ヱ', 'ゑ'], ".--.."), /* ye */
        (vec!['ヒ', 'ひ'], "--..-"), /* hi */
        (vec!['モ', 'も'], "-..-."), /* mo */
        (vec!['セ', 'せ'], ".---."), /* se */
        (vec!['ス', 'す'], "---.-"), /* su */
        (vec!['ン', 'ん'], ".-.-."), /* n  */
        //
        // かな(濁音) / KANA - a voice sound
        /* characters with turbidity suffix */
        // jscwlib.jsから転記。
        (vec!['゛'], ".."),             /* "  */
        (vec!['ガ', 'が'], ".-.. .."),  /* ga */
        (vec!['ギ', 'ぎ'], "-.-.. .."), /* gi */
        (vec!['グ', 'ぐ'], "...- .."),  /* gu */
        (vec!['ゲ', 'げ'], "-.-- .."),  /* ge */
        (vec!['ゴ', 'ご'], "---- .."),  /* go */
        (vec!['ザ', 'ざ'], "-.-.- .."), /* za */
        (vec!['ジ', 'じ'], "--.-. .."), /* zi */
        (vec!['ズ', 'ず'], "---.- .."), /* zu */
        (vec!['ゼ', 'ぜ'], ".---. .."), /* ze */
        (vec!['ゾ', 'ぞ'], "---. .."),  /* zo */
        (vec!['ダ', 'だ'], "-. .."),    /* da */
        (vec!['ヂ', 'ぢ'], "..-. .."),  /* di */
        (vec!['ヅ', 'づ'], ".--. .."),  /* du */
        (vec!['デ', 'で'], ".-.-- .."), /* de */
        (vec!['ド', 'ど'], "..-.. .."), /* do */
        (vec!['バ', 'ば'], "-... .."),  /* ba */
        (vec!['ビ', 'び'], "--..- .."), /* bi */
        (vec!['ブ', 'ぶ'], "--.. .."),  /* bu */
        (vec!['ベ', 'べ'], ". .."),     /* be */
        (vec!['ボ', 'ぼ'], "-.. .."),   /* bo */
        //
        // かな(半濁音) / KANA - P-sound
        /* characters with semi-turbidity suffix */
        // jscwlib.jsから転記。
        (vec!['゜'], "..--."),             /* *  */
        (vec!['パ', 'ぱ'], "-... ..--."),  /* pa */
        (vec!['ピ', 'ぴ'], "--..- ..--."), /* pi */
        (vec!['プ', 'ぷ'], "--.. ..--."),  /* pu */
        (vec!['ペ', 'ぺ'], ". ..--."),     /* pe */
        (vec!['ポ', 'ぽ'], "-.. ..--."),   /* po */
        //
        // かな記号
        // jscwlib.jsから転記。注記追加変更
        (vec!['－'], ".--.-"),  /* 長音とハイフンは同じモールス符号 */
        (vec!['ー'], ".--.-"),  /* 長音とハイフンは同じモールス符号 */
        (vec!['（'], "-.--.-"), /* 英文字の')'と同じモールス符号 */
        (vec!['）'], ".-..-."), /* 英文字の'"'と同じモールス符号 */
        (vec!['、'], ".-.-.-"), /* 英文字の'.'と同じモールス符号 */
        (vec!['」'], ".-.-.."), /* \n */
        //
        // 一文字化するためのシフト用
        (vec!['<'], "<"),
        (vec!['>'], ">"),
    ];

    for (x, v) in tbl {
        for k in x {
            table.insert(k, v);
        }
    }

    return table;
}
