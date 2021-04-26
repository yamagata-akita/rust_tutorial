// Rustの言語仕様では構造体や列挙型の内部表現を定義していない
// 構造体がメモリにどのように買う脳されるかを調べる
// 構造体のサイトと各フィールドが格納されるアドレスを表示する

#[derive(Default)]
struct A {f0: u8, f1: u32, f2: u8}

fn main() {
    let a: A = Default::default();
    println!("struct A ({} bytes)\n f0: {:p}\n f1: {:p}\n f2: {:p}\n",
        std::mem::size_of::<A>(), &a.f0, &a.f1, &a.f2);
}