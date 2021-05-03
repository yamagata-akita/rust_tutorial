// 構造体や列挙型にCopyトレイト(std::marker::Copy)を実装すると、値がムーブ！するのではなく、
// コピーされるようになる
// let p2 = p1というシンタックスが、型によってはコピーセマンティクスに変わる

// 構造体や列挙型が以下のすべての条件を満たすとき、Copyトレイトを実装できる
// 1.その型(構造体や列挙型)のすべてのフィールドの型がCopyトレイトを実装している
//   例えばi32型はCopyトレイトを実装している
// 2.その型自身とすべてのフィールドの型がデストラクタ(Dropトレイト)を実装していない
//   ヒープ領域を使用するデータ型(Box<T>, Vec<T>, Stringなど)はデストラクタを持つため、
//   フィールドにそれらの型を持つ場合はCopyトレイトを実装不可
// 3.その型自身がCloneトレイト(std::clone::Clone)を実装している

#[derive(Copy, Clone, Debug)]
struct Parent(usize, Child, Child); // Parentはusizeに加えてChildを2つ持つ

#[derive(Copy, Clone, Debug)]
struct Child(usize);


// デストラクタの実装はコメントアウト
/*
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
*/


fn main() {
    // 所有権の譲渡

    let p1 = Parent(1, Child(11), Child(12));
    let p2 = p1;        // p1が所有する値がコピーされ、コピーされたほうをp2が所有する
    println!("p2: {:?}", p2);
    println!("p1: {:?}", p1);

}