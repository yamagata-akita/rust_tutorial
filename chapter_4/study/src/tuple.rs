fn main() {

    // タプル型
    // 要素の数を要素数またはアリティとよぶ
    // 各要素の型は異なっていても良い

    // 要素へのアクセス
    // フィールド名を使う方法と、パターンマッチで分解する方法がある

    // フィールド名を使う
    let t1 = (88, true);

    // フィールド0の要素を取り出す
    assert_eq!(t1.0, 88);

   // フィールド名にはコンパイル時の定数のみ使える。変数は不可
   // let i = 0;
   // let t1a = t1.i; 
   // →コンパイルエラー
   // no field 'i' on type '({integer}, bool)'
   // ({整数}, bool)型にはiという名のフィールドはありません

   // 要素数を書き換えるときは、変数をmutで可変にする
   let mut mt1 = (88, true);
   mt1.0 += 100;
   assert_eq!(mt1, (188, true));

   // パターンマッチで分解する
   let (n1, b1) = (88, true);
   assert_eq!(n1, 88);
   assert_eq!(b1, true);

   let ((x1, y1), (x2, y2)) = ((0, 5), (10, -1));
   assert_eq!(x1, 0);
   assert_eq!(y1, 5);
   assert_eq!(x2, 10);
   assert_eq!(y2, -1);

   // 不要な値はアンダースコアを使うと無視できる
   // let ((x1, y1), _) = ((0, 5), (10, -1));

   // 可変
   let mut mt2 = ((0, 5), (10, -1));

   // 要素を指す可変の参照を得るためにref mutを追加する
   let ((ref mut x1_ptr, ref mut y1_ptr), _) = mt2;

   // *をつけることでポインタが指すアドレスにあるデータにアクセスできる
   *x1_ptr += 3;
   *y1_ptr *= -1;

   assert_eq!(mt2, ((3, -5), (10, -1)));

}