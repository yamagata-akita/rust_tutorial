// 構造体を定義する
// println!の{:?}で表示できるように、Debugトレイトを自動導出しておく

#[derive(Debug)]
struct Parent(usize, Child, Child); // Parentはusizeに加えてChildを2つ持つ

#[derive(Debug)]
struct Child(usize);


// 構造体や列挙型では値が破棄される直前に終了処理を行うための
// 特別な関数を定義できる
// この関数はデストラクタと呼ばれ、Dropトレイト(std::ops::Drop)を通して実装できる

use std::ops::Drop;

// Parent構造体にデストラクタを実装する
impl Drop for Parent {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

// Child構造体にデストラクタを実装する
impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

// dropメソッドは値が破棄される直前に暗黙的に呼ばれる
// 同じブロック内ではあとから導入された変数が先にスコープを抜ける(p3→p1の順)


fn main() {
    let p1 = Parent(1, Child(11), Child(12));
    // まずChild(11)とChild(12)を作り、それらをParent(1, ..)にもたせている
    // そのため、Child(11)とChild(12)の所有者はParent(1, ..)になる
    // その後、Parent(1, ..)は変数p1に持たせた
    // そのため、Parent(1, ..)の所有者はp1

    {
        // ブロックを作り、p2はその中で導入する
        let p2 = Parent(2, Child(21), Child(22));
        println!("(a) p1: {:?}, p2: {:?}", p1, p2); // (a)の時点
    }

    println!("(b) p1: {:?}", p1);                   // (b)の時点

    let p3 = Parent(3, Child(31), Child(32));
    println!("(c) p1: {:?}, p3: {:?}", p1, p3);     // (c)の時点
}