fn main() {
   // 配列型
   // 配列の要素数を長さと呼ぶ。配列の方は要素の型と長さで表す 
   // [bool; 3], [f64; 4], [[char;2]; 2]など

   let bool_3 = [false, true, false];
   let f64_4 = [0.0, -1.0, 1.0, 0.5];

   // len()で配列の長さを得られる
   assert_eq!(f64_4.len(), 4);

   // 長さ100の配列を作り、全要素を0i32で初期化する
   let i32_100 = [0; 100];
   assert_eq!(i32_100.len(), 100);

   // 配列の長さは実行時に指定できない
   // let size = 100;
   // let a1 = [0; size];
   // →コンパイルエラー

   // ベクタなら実行時に長さを指定可能
   let size = 100;
   let mut v1 = vec![0; size];
   assert_eq!(v1.len(), 100);

   // ベクタには要素を追加したり、削除したりできる
   v1.push(1);
   assert_eq!(v1.len(), 101);
   assert_eq!(v1.pop(), Some(1));
   assert_eq!(v1.len(), 100);


   // 要素へのアクセス(インデックスとイテレータの2通り)
    let array1 = ['H', 'e', 'l', 'l', 'o'];
    assert_eq!(array1[1], 'e');
    
    let mut array2 = [0, 1, 2];
    array2[1] = 10;
    assert_eq!(array2, [0, 10, 2]);

    // インデックスは定数でなくてもOK
    let mut index = 0;
    assert_eq!(array2[index], 0);
    index += 1;
    assert_eq!(array2[index], 10);

    // インデックスの範囲は常にチェックされ、コンパイル時にわかる場合はコンパイルエラー、わからなければ実行時にパニック
    let array3 = [0, 1];
    // array3[2];
    // →エラー
    // let index = 2;
    // array3[index];
    // →コンパイルエラーにはならず、実行時にパニックする

    // 値を取り出すときは、より安全な(パニックしない)get()メソッドも使える
    assert_eq!(array3.get(1), Some(&1)); // get()はインデックスが範囲内のときはSome(&値)を返す
    assert_eq!(array3.get(2), None); // さもなければNoneを返す

    // イテレータを使う
    let array4 = ['a'; 50];
    // iter()で要素が不変のイテレータを作成
    for ch in array4.iter() {
        // print!("{}", *ch);
    }

    let mut array5 = [1; 50];
    // iter_mut()で要素が可変のイテレータを作成
    for n in array5.iter_mut() {
        *n *= 2;
    }
    assert_eq!(array5, [2; 50]);


    // 配列に対してインデックスによるアクセスやlen(), get(), iter()などの操作を行うと、
    // その配列をスライスへと暗黙的に型強制(type coercion)する仕組みになっている

    // スライス
    
    // この関数は&[char]型のスライスを引数に取り、その情報を表示する
    fn print_info(name: &str, sl: &[char]) {
        println!(" {:9} - {}, {:?}, {:?}, {:?}",
            name,
            sl.len(),   // 長さ(バイト数)   usize型
            sl.first(), // 最初の要素       Option<char>型
            sl[1],      // 2番目の要素      char型
            sl.last()   // 最後の要素       Option<char>型
        );
    }

    // 配列
    let a1 = ['a', 'b', 'c', 'd'];      // 参照元のデータ [char; 4]型
    println!("a1, {:?}", a1);

    print_info("&a1[..]", &a1[..]);     // &[char]型。全要素のスライス

    print_info("&a1", &a1);             // 同上

    print_info("&a1[1..3]", &a1[1..3]); // b,cを要素とする長さ2のスライス

    // ベクタ
    let v1 = vec!['e', 'f', 'g', 'h'];  // 参照元のデータ Vec<char>型
    println!("\nv1: {:?}", v1);

    print_info("&v1[..]", &v1[..]);     // &[char]型。全要素のスライス
    print_info("&v1", &v1);             // 同上
    print_info("&v1[1..3]", &v1[1..3]); // &[char]型。f,gを要素とする長さ2のスライス

    // ミュータブルなスライス
    let mut a1 = [5, 4, 3, 2];  // 配列 [i32, 4]型
    let s1 = &mut a1[1..3];     // 可変のスライス。&mut[i32]型
    s1[0] = 6;
    s1[1] *= 10;
    s1.swap(0, 1);
    assert_eq!(s1, [30, 6]);

    // 参照元の配列の内容を確認
    assert_eq!(a1, [5, 30, 6, 2]);  // スライスを通じて配列の内容が変更された

    // スライスの操作
    let a2: [i32; 0] = [];
    let s2 = &a2;            // 不変のスライスを作成
    assert!(s2.is_empty());
    assert_eq!(s2.len(), 0);
    assert_eq!(s2.first(), None);

    let a3 = ["zero", "one", "two", "three", "four"];
    let s3 = &a3[1..4];
    assert!(!s3.is_empty());
    assert_eq!(s3.len(), 3);
    assert_eq!(s3.first(), Some(&"one"));

    assert_eq!(s3[1], "two");
    // assert_eq!(s3[3], "?");          // 4番目の要素は存在しない。panicする
    assert_eq!(s3.get(1), Some(&"two"));
    assert_eq!(s3.get(3), None);        // 4番目の要素は存在しない。None

    assert!(s3.contains(&"two"));       // "two"を要素に持つ
    assert!(s3.starts_with(&["one", "two"])); // "one", "two"ではじまる
    assert!(s3.ends_with(&["two", "three"])); // "two", "three"で終わる

    
    let mut a4 = [6, 4, 2, 8, 0, 9, 4, 3, 7, 5, 1, 7];

    // 一部の要素を昇順にソートする
    &mut a4[2..6].sort();
    assert_eq!(&a4[2..6], &[0, 2, 8, 9]);

    // スライスを2つの可変スライスへ分割する
    let (s4a, s4b) = &mut a4.split_at_mut(5);

    // 前半を逆順にする
    s4a.reverse();
    assert_eq!(s4a, &[8, 2, 0, 4, 6]);

    // 後半を昇順にソートする
    s4b.sort_unstable();
    assert_eq!(s4b, &[1, 3, 4, 5, 7, 7, 9]);

    // sort()とsort_unstable()のちがい
    // sort()は安定ソートなので同順なデータのソート前の順序がソート後も保存される
    // sort_unstable()は安定ソートではないが、一般的にsort()より高速

    // &mutを省略しても同じ。型強制によって自動的にスライスが作られる
    a4[2..6].sort();
    // let (s4a, s4b) = a4.split_at_mut(5);
    
}