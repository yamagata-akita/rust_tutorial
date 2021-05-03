#[derive(Debug)]
struct Parent(usize, Child, Child); // Parentはusizeに加えてChildを2つ持つ

#[derive(Debug)]
struct Child(usize);


use std::ops::Drop;

impl Drop for Parent {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

// 借用: 所有権を渡さずに値を貸し出す
// 関数に引数として与えると所有権がムーブしてしまう
fn f0(p: Parent) {
    println!("p: {:?}", p);
}

// Parentへの不変の参照を引数に取る
fn f1(p: &Parent) {
    println!("p: {:?}", p);
}

// Parentへの可変の参照を引数に取る
fn f2(p: &mut Parent) {
    p.0 += 1;
}

fn main() {
    // 所有権の譲渡

    let mut p1 = Parent(1, Child(11), Child(12));
    let p2 = p1;                                    // 値の所有権をp1からp2にムーブする
    // p1からp2にムーブされた時点で、コンパイラは変数p1から値が立ち退いた(ムーブアウトした)とみなされる
    // p1は初期化されていないときと同じように扱われる
    println!("p2: {:?}", p2);
    // println!("p1: {:?}", p1);                    // p1は値の所有権を失ったためアクセス不可
    // → error[E0382]: borrow of moved value: `p1`

    p1 = Parent(2, Child(21), Child(22));           // p1を別の値に束縛する
    println!("p1: {:?}", p1);                       // p1は別の値の所有権を持つためアクセスできる


    // 借用
    // 関数に引数として与えると所有権がムーブしてしまう
    let p3 = Parent(1, Child(31), Child(32));
    f0(p1); // 所有権が関数の引数にムーブする
    // println!("p1: {:?}", p1); // p1は値の所有権を失ったのでアクセス不可
    // -> error[E0382]: borrow of moved value: `p1`

    let mut p1 = Parent(1, Child(11), Child(12));
    f1(&p1);                                        // f1には所有権をムーブせず、不変の参照を渡す。&p1はf1から抜けるときにライフタイムが尽きて破棄される
    f2(&mut p1);                                    // f2には所有権をムーブせず、可変の参照を渡す
    println!("p1: {:?}", p1);                       // p1は値の所有権を失っていないのでアクセスできる


}