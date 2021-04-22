fn main() {
    // ベクタ(std::vec::Vec<T>)
    // プリミティブ型の配列との違い
    // 要素の追加や削除ができる
    // 配列はスタックに置かれるが、ベクタはヒープに置かれる

    // ベクタを初期化するにはvec![]マクロを使う

    let v1 = vec![false, true, false];  // Vec<bool>型
    let v2 = vec![0.0, -1.0, 1.0, 0.5]; // Vec<f64>型

    assert_eq!(v2.len(), 4);

    // 長さ100のベクタを作り、全要素を0i32で初期化する
    let v3 = vec![0; 100];
    assert_eq!(v3.len(), 100);

    // ベクタは入れ子にできる。子の要素数はそれぞれが異なってもかまわない
    let v4 = vec![vec!['a', 'b', 'c'], vec!['d']]; // Vec<Vec<char>>型

    // ベクタは同じ型の要素の並び。異なる方の要素は持てない
    // let v5 = vec![false, 'a'];
    // → error[E0308]: mismatched types

    let mut v6 = vec!['a', 'b', 'c'];               // Vec<char>型
    v6.push('d');                                   // 最後尾に値を追加
    v6.push('e');
    assert_eq!(v6, ['a', 'b', 'c', 'd', 'e']);

    assert_eq!(v6.pop(), Some('e'));                // 最後尾から値を取り出し
    v6.insert(1, 'f');                              // インデックス1の位置に要素を挿入
    assert_eq!(v6.remove(2), 'b');                  // インデックス2の要素を削除。削除した値が戻り値になる

    assert_eq!(v6, ['a', 'f', 'c', 'd']);

    let mut v7 = vec!['g', 'h'];
    v6.append(&mut v7);                             // v6の最後尾にv7の全要素を追加
    assert_eq!(v6, ['a', 'f', 'c', 'd', 'g', 'h']);
    assert_eq!(v7, []);                             // v7は空になる

    let a8 = ['i', 'j'];                            // 配列a8
    v6.extend_from_slice(&a8);                      // v6の最後尾にa8の全要素を追加
    assert_eq!(v6, ['a', 'f', 'c', 'd', 'g', 'h', 'i', 'j']);
    assert_eq!(a8, ['i', 'j']);                     // a8は変化なし(a8の要素がコピーされた)

    // 空のベクタを作るときはnew()メソッドを使う
    // 大まかな要素数がわかっているときはwith_capacity(要素数)メソッドがおすすめ
    // ベクタに要素を追加する際のメモリ再割当てのオーバヘッドが削減できる
}