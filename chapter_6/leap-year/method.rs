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
}

fn main() {
    let circle1 = Circle { radius: 10 };
    println!("Circle1's diameter: {}", circle1.diameter());
}