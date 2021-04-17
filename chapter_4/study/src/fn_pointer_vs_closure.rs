fn main() {

    let x = 4;

    // クロージャを定義する。するとxがクロージャの環境に補足される(キャプチャされる)
    let adder = |n| n + x;
    assert_eq!(adder(2), 4 + 2);

    let mut state = false;
    // 別のクロージャを定義する。このクロージャは引数を取らない
    let mut flipflop = || {
        // stateが補足される
        state = !state;
        state
    };

    // クロージャを呼ぶたびに変える値が反転する
    assert_eq!(flipflop(), true); // true
    assert_eq!(flipflop(), false); // false
    assert_eq!(flipflop(), true); // true

    // クロージャが返す値だけでなく、stateの値も変化している
    assert_eq!(state, true); // true


    /*
    // 上のコードをコンパイルすると、コンパイラはクロージャの環境を表現するために匿名の型(匿名の構造体)を生成する
    // adderの環境
    struct adder_env {
        x_ref: &i32, // xの不変の参照
    }

    // flipflopの環境
    struct flipflop_env {
        state_ref: &mut bool, // stateの可変の参照
    }
    */
    

    let b = 5;

    // クロージャは1つ1つが独自の匿名の型を持つため、変数fの型はこのクロージャの匿名型になる
    let mut f = |a: i32| a * 3 + b;
    // 別のクロージャでは変数fと型が合わず、コンパイルエラーになる
    // f = |a: i32| a * 4 + b;
    // note: no two closures, even if identical, have the same type
    // (2つのクロージャは、たとえ見た目が同じでも、同じ型を持つことはない)


    // 環境になにも補足しないクロージャは空の環境を持つため、関数ポインタ型になることができる
    let mut f: fn(i32) -> i32 = |n| n * 3;
    assert_eq!(f(-42), -126);

    // 環境がなにかを補足するクロージャは関数ポインタ型になれない
    let x = 4;
    // f = |n| n * x;

    // クロージャが要求される場面で関数ポインタを渡す
    let v = vec!["I", "love", "Rust!"]
        .into_iter()
        .map(|s| s.len()) // &str型の引数sを取るクロージャ
        .collect::<Vec<_>>();

    assert_eq!(v, vec![1, 4, 5]);
}