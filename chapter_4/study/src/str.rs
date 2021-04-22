fn main() {

    // 文字列スライス型(str型)
    // &strか&mut strと表記する
    // strリテラルはダブルクオートで囲み、型は &'static str
    // 'staticはライフタイム指定子と呼ばれる

    let s1 = "abc1";        // &'static str型
    let s2 = "abc2";
    assert!(s1 < s2);
    assert!(s1 != s2);

    let s3 = "文字列を複数行に渡って書くと
        改行やスペースが入る";
    let s4 = "行末にバックスラッシュをつけると\
        改行などが入らない";
    
    assert_eq!(s3, "文字列を複数行に渡って書くと\n        改行やスペースが入る");
    assert_eq!(s4, "行末にバックスラッシュをつけると改行などが入らない");

    let s5 = "文字列に\"と\\を含める";  // バックスラッシュでエスケープ
    let s6 = r#"文字列に"と\を含める"#; // raw文字列リテラル。正規表現などに便利
    assert_eq!(s5, s6);

    let s7 = r###"このように#の数を増やすと"##"があっても大丈夫"###;
    assert_eq!(s7, "このように#の数を増やすと\"##\"があっても大丈夫");

    let s8 = "絵文字\u{1f600}も使える";
    print!("{}", s8);

    let fruits = "あかりんご, あおりんご\nラズベリー, ブラックベリー";

    // lines()メソッドは改行コード(\n)を含む文字列から1行ずつ
    // 取り出せるイテレータを作る
    let mut lines = fruits.lines();
    // イテレータのnext()メソッドで次の行を得る
    let apple_line = lines.next();
    assert_eq!(apple_line, Some("あかりんご, あおりんご"));
    assert_eq!(lines.next(), Some("ラズベリー, ブラックベリー"));
    // 次の行がないからNoneがかえる
    assert_eq!(lines.next(), None);

    // りんごの行(Some(..))の中身を取り出す
    if let Some(apples) = apple_line {
        // 「あか」で始まるかチェック
        assert!(apples.starts_with("あか"));
        // 「りんご」の文字を含むかチェック
        assert!(apples.contains("りんご"));
        // 「あお」が最初に出現する位置(UTF-8表現で何バイトめ)を得る
        assert_eq!(apples.find("あお"), Some(17)); // 0はじまりなので18バイト目

        // 文字列をカンマで分割するイテレータを作る
        let mut apple_iter = apples.split(",");
        assert_eq!(apple_iter.next(), Some("あかりんご"));
        let green = apple_iter.next();
        assert_eq!(green, Some(" あおりんご"));

        // Some(..)の内容にstrのtrim()メソッドを適用して余白を取り除く
        assert_eq!(green.map(str::trim), Some("あおりんご"));
        assert_eq!(apple_iter.next(), None);

    } else {
        unreachable!(); // もしここに到達したらパニックで強制終了する
    }

    // strの長さ
    // len()メソッドで取得。文字数ではなくUTF-8のバイト数となる
    let ls1 = "a";
    let ls2 = "あ";
    assert_eq!(ls1.len(), 1);
    assert_eq!(ls2.len(), 3);

    // 何文字目ではなく、UTF-8の何バイト目から何バイト目という取り出し方をする
    let s = "abcあいう";
    assert_eq!(s.get(0..1), Some("a"));
    assert_eq!(s.get(3..6), Some("あ"));
    assert_eq!(s.get(3..4), None); // UTF-8として解釈できない場合

    // chars()でstrからchar型のイテレータを取り出せる
    let ss = "かか\u{3099}く";
    println!("{}", ss);

    let mut iter2 = ss.chars();
    assert_eq!(iter2.next(), Some('か'));
    assert_eq!(iter2.next(), Some('か'));
    assert_eq!(iter2.next(), Some('\u{3099}'));
    assert_eq!(iter2.next(), Some('く'));
    assert_eq!(iter2.next(), None);


    // 可変のstr
    // &mut strは可変スライス経由のアクセスのため、UTF-8バイト列の要素は変更できる
    // 要素の追加、削除はできないという制限あり

    // 文字列リテラル(&'static str)から&mut strは直接得られない
    // まず文字列リテラルをStringへ変換し、そこから&mut strを取り出す
    let mut s1 = "abcあいう".to_string(); // String型

    // &mut strを得る。これはStringが持つUTF-8バイト列を指す可変スライス
    let s2 = s1.as_mut_str();

    // 英小文字を大文字に変更
    s2.make_ascii_uppercase();
    assert_eq!(s2, "ABCあいう");

    // &mut strのUTF-8バイト列を直接操作して"あ"(3バイト)を"*a*"に変更する
    let b = unsafe { s2.as_bytes_mut() };
    b[3] = b'*';
    b[4] = b'a';
    b[5] = b'*';

    assert_eq!(s1, "ABC*a*いう");
}