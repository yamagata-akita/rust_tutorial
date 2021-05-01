// メソッド
// 関数の一種
// 構造体、列挙型、トレイトの中などに定義された関数の一種
// メソッドでは、メソッドの関連付けられたインスタンス(=レシーバ)の情報を使うことができる

struct Circle {
    radius: u32,
}

// 構造体のメソッドの定義は、構造体の定義の中ではなく、implブロックの中に書く

impl Circle {
    fn diameter(&self) -> u32 {
        self.radius * 2
    }

    // 関連関数
    // 構造体などデータ型そのものに関数を関連付けることができる

    //small_circle関連関数
    fn small_circle() -> Circle {
        Circle { radius: 1 }
    }
}

fn main() {
    let circle1 = Circle { radius: 10 };
    println!("Circle1's diameter: {}", circle1.diameter());

    // Circleの関連関数small_circleの呼び出し
    // 関連関数の実行には::を用いる
    let circle2 = Circle::small_circle();
    println!("Circle2's diameter: {}", circle2.diameter());
}