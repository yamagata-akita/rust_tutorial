fn main() {
    // String型
    // プリミティブ型のstr型と対をなす型。
    // strは主に&strとしてアクセスされるため不変だった。
    // Stringは文字の変更、追加や削除ができる
    // &strとStringの関係は、不変のスライスとベクタのそれに似ている
    // 実データはUTF-8形式でエンコードされており、ヒープに格納される

    // strリテラルからStringをつくる
    let mut s1 = "ラズベリー".to_string();
    let mut s2 = String::from("ブラックベリー");
    let s3 = "ストロベリー".to_owned();

    s1.push_str("タルト");
    assert_eq!(s1, "ラズベリータルト");

    s2.push('と');          // Stringにcharを追加する

    // push_str()が受け付けるのは&str型のみ
    // s2.push_str(s3);
    // → error: mismatched types
    // expected &str, found struct 'std::string::String'

    // &をつけると型強制によって&Stringから&strに変換される
    s2.push_str(&s3);
    assert_eq!(s2, "ブラックベリーとストロベリー");

    // 数値型からStringをつくる
    let i = 42;
    assert_eq!(i.to_string(), "42");

    // 文字列から数値型の値を作る
    let s1 = "42";
    assert_eq!(s1.parse::<i32>(), Ok(42));

    // charやバイト列からStringへ
    let cs = ['t', 'r', 'u', 's', 't']; // [char; 5]型
    assert_eq!(cs.iter().collect::<String>(), "trust");
    assert_eq!(&cs[1..].iter().collect::<String>(), "rust");

    // &strは参照先のUTF-8バイト列よりも短い期間しか生存できないため、
    // 関数から返すときは注意が必要
    // この関数は引数として&str型の名前を取り、&str型の"Hello, 名前！"を返す
    // fn f1(name: &str) -> &str {
        // let s = format!("Hello, {}!", name); // format!はStringを作る
        // &s                              // Stringから&strを作成し、戻り値として返す
        // → コンパイルエラー: 's' does not live long enough

        // 原因
        // format!で新たに作ったStringが、関数を抜けるときにライフタイムが尽きて削除されること
    // }

    // 正しい方法 → format!で作ったStringをそのまま返すこと
    fn f1(name: &str) -> String {
        format!("Hello, {}!", name)
    }
}