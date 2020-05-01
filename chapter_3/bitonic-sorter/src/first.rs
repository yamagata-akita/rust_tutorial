// pubはこのsort関数が他のモジュールからアクセスできることを示す
// 引数Xの型 &mut [u32]について
// &は値をポインタ経由で借用することを示す
// mutは値が変更可能であることを示す
// u32型は32ビット符号なし整数
// [u32]型はu32のスライス(1次元配列のようなもの)
pub fn sort(x: &mut [u32], up: bool) {
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        sort(&mut x[..mid_point], true);
        sort(&mut x[mid_point..], false);
        sub_sort(x, up);
    }
}


fn sub_sort(x: &mut [u32], up: bool) {
    unimplemented!();
}


fn compare_and_swap(x: &mut [u32], up: bool) {
    unimplemented!();
}
