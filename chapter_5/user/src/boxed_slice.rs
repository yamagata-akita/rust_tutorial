fn main() {
    // 4要素のベクタVec<i32>を作り、要素を1つ足して5要素に拡張する
    let mut v1 = vec![0, 1, 2, 3];
    v1.push(4);
    println!("v1 len: {}, capacity: {}", v1.len(), v1.capacity());
    // 5要素だが、8要素分のメモリを確保している

    // Box<[i32]>に変換する。余分なメモリを持たなくするためにVecのshrink_to_fit()
    // メソッドが実行されてからBox化される。
    let s1 = v1.into_boxed_slice();
    // 余分なメモリを持っていないことを確認するためVec<i32>に戻す
    let v2 = s1.into_vec();
    println!("v2 len: {}, capacity: {}", v2.len(), v2.capacity());


}