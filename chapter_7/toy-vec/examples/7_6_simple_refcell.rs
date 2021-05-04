// 内側のミュータビリティ
// コンパイル時の借用チェックを迂回してデータを可変にする仕組み

struct A {
    c: char,
    s: String,
}

use std::cell::RefCell;

struct B {
    c: char,
    s: RefCell<String>,     // StringをRefCellで包む
}


fn main() {
    let a = A { c: 'a', s: "alex".to_string() };
    let r = &a;     // 不変の参照を作る
    // r.s.push('a');  // 不変の参照経由でフィールドを変更しようとすると、コンパイルエラーになる
    // → error

    let b = B {c: 'a', s: RefCell::new("alex".to_string()) };
    let rb = &b;
    rb.s.borrow_mut().push('a');        // フィールドsのデータに対する可変の参照を取る

    {
        let rbs = b.s.borrow();         // 不変の参照を取る
        println!("{}", &*rbs);          // alexa

        // RefCellでは他の参照が有効な間に可変の参照を取ろうとすると実行時にパニックする
        // b.s.borrow_mut();               // この時点で不変の参照rbsがまだ有効
        // panic
        // try_borrow_mutならパニックせずErrを返す
        assert!(b.s.try_borrow_mut().is_err());
    }
    // rbsはここでスコープを抜ける
    assert!(b.s.try_borrow_mut().is_ok());
}