use toy_vec::ToyVec;

fn main() {
    let mut v = ToyVec::new();
    v.push("Java Finch".to_string());
    v.push("Budgerigar".to_string());
    let e = v.get(1);
    assert_eq!(e, Some(&"Budgerigar".to_string()));


    // 値が共有されている間(不変の参照が有効な間)は値の変更を許さない
    let mut v: ToyVec<usize> = ToyVec::new();
    v.push(100);
    let e = v.get(0);                           // 不変の参照(不変の借用)を取得
    // v.push(200);                                // ベクタを変更する(pushは可変の参照を取る)
    // →エラー
    assert_eq!(e, Some(&100));                  // ここで不変の参照にアクセス
}