fn main() {
    // 束縛とは

    // let文: 変数を導入する
    // let パターン[: 型] [= 初期化式];
    let date_string = "2019-01-26"; // 型を省略できる
    let pi: f64 = 3.14;             // 型を明示しても良い
    let (a, b) = (19, 79);           // パターンは単なる変数ではない

    // 変数と値とを結びつけることを「変数を値に束縛する」という


    // ミュータビリティ
    // let mut パターン[: 型] [= 初期化式];
    let mut mutable_string = String::from("String");    // 文字列に束縛
    mutable_string = String::from("Hello");             // 別の文字列に束縛
    mutable_string.push_str(", world!");                // 文字列を変更する操作
    // mutable_string = 2019;                           // →エラー(異なる型の値に束縛し直すことはできない)


    // シャドウイング
    // 変数は同じ名前で複数作ることができる
    // 新しく導入した変数で前に導入した変数を隠してしまうことをシャドウイングと呼ぶ。
    fn shadowing_example() {
        let x = 10;
        let x = 20;
        let x = "String";

        println!("{}", x);

        {
            let x = 30;
            println!("{}", x);
        }

        println!("{}", x);
    }
    shadowing_example();

    // 定数
    // const 名前: 型 = 定数式;
    // const文では、定数の型を明示しなければならない

    // スタティック変数
    // static [mut] 名前: 型 = 定数式;
    // static文で定義されたスタティック変数は、定数と異なり、
    // 値が埋め込まれることはなく、使われるたびに参照される


    // 戻り値のあるif式
    // 戻り値は同じ型でなければならない
    let a = 12;
    let even_or_odd = if a % 2 == 0 {
        "an even"
    } else {
        "an odd"
    };
    println!("{} is {} number", a, even_or_odd);


    // match式とパターン
    // match 検査される値 {
    //     パターン1 => 式1,
    //     パターン2 => 式2,
    //     パターン3 => 式3,
    //     ...
    // }

    let value = 100;

    match value {
        1 => println!("One"),
        10 => println!("Ten"),
        100 => println!("One hundred"),
        _ => println!("Something else"),
    }


    let unknown = Some("Apple");

    let string = match unknown {
        Some(something) => String::from("Hi, ") + something,
        None => String::from("Nothing"),
    };
    println!("{}", string);


    let ten = 10;
    let ten_reference = &ten;

    match ten_reference {
        number => assert_eq!(&10, number),  // numberは参照
    };
    
    match ten_reference {
        &number => assert_eq!(10, number),  // numberは参照ではない
    }


    let number = 42;

    let string = match number {
        // パターンの連結
        1 | 2 | 3 => "One or two or three",
        // 範囲のパターン
        40 ... 50 => "From 40 to 50",
        _ => "Something else",
    };

    println!("{}", string);


    let string = Some("This is a very long string");

    // パターンには条件をつけることができる
    // パターンにつける条件のことをガードと呼ぶ
    let message = match string {
        Some(s) if s.len() >= 10 => "Long string",
        Some(_) => "String",
        None => "Nothing",
    };

    println!("{}", message);
}
