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
    }
}