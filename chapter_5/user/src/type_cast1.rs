// 型変換

// ・型キャスト(type cast)
//   as 型を使用した明示的な型変換。データ変換を伴い、スカラ型同士を変換できる

// ・Transmute
//   コンパイラ・イントリンシック(組み込み命令)であるstd::mem::transmute
//   を使用した明示的かつアンセーフな型変換。データ変換を伴わない

// ・型強制(type coercion)
//   コンパイラによる暗黙的な型変換。データ変換を伴う

fn main() {
    // 型キャスト
    let i1 = 42;        // i32型
    let f1 = i1 as f64; // i32型からf64型へキャスト

    let c1 = 'a';
    assert_eq!(97, c1 as u32);  // char型からu32へキャスト

    // 型キャストは桁溢れ(オーバーフロー)をチェックしないことに注意
    let i2 = 300;       // i32型
    let u1 = i2 as u8;  // u8型へキャスト

    // 300はu8型の最大値を超えているので桁溢れして44になる
    assert_eq!(44, u1);

    // asはスカラ型投資の型変換だけをサポートしており、タプルや配列のような
    // 複合型の変換には使えない
    let t1 = ('a', 42);
    let v1 = vec![b'h', b'e', b'1', b'1', b'o'];    // Vec<u8>型
    // let t2 = t1 as (u32, u8);
    // →error[E0605]: non-primitive cast: '(char, i32)' as '(u32, u8)'

    // このような場合は、要素を1つずつキャストする
    let t3 = (t1.0 as u32, t1.1 as u8);
    let v3 = v1.iter().map(|&n| n as u16).collect::<Vec<u16>>();


    // Tranmute(std::mem::transmute)
    // 明示的かつアンセーフな型変換
    // メモリ上の表現(ビット列)のサイズさえ同じなら、どんな型同士でも変換できる
    let p1 = Box::new(10);  // Box<i32>型

    // boxポインタを生ポインタ*mut i32型に変換したいが型キャストできない
    // let p2 = p1 as *mut i32;
    // →erro : non-primitive cast:...

    // Boxポインタと*mutポインタはどちらも同じビット幅なのでtransmuteできる
    let p3: *mut i32 = unsafe {std::mem::transmute(p1)};

    // transmuteはコンパイラが追跡している情報のうち、値の方に関する情報を変更するが、
    // 値自体の変換はしない
    // 今回の場合、Box<i32>型と認識していた値を、*mut i32型だと認識するようになっただけ

    // 型キャストとtransmuteの違い
    let f1 = 5.6789e+3_f32; //5678.9

    // f32型からi32型へ型キャストする。小数点以下は切り捨てられる
    let i1 = f1 as i32;
    println!("{}", i1); //5678

    // f32型からi32型へtransmuteする
    let i2: i32 = unsafe {std::mem::transmute(f1)};
    println!("{}", i2); // 浮動小数点数を整数として再解釈した値1169258291


    // 型強制
    // コンパイラが行う暗黙的な型変換

    // 整数リテラル3,4,5は通常i32型と解釈されるが、
    // 型アノテーション(型注釈)によってu8型へと型強制されている
    let v1: Vec<u8> = vec![3, 4, 5];

    // Vec<u8>からスライス&[u8]型へ型強制されることで、
    // スライスに備わったfirst(&self)メソッドが使用できる
    assert_eq!(Some(&3u8), v1.first());

    // もし型強制がなかったらこう書くことになる
    // assert_eq!(Some(&3u8), (&v1[..]).first());

    let mut s1 = String::from("Type coercion ");
    let s2 = String::from("is actually easy.");

    // push_str()のシグネチャはpush_str(self: &mut String, s: &str)
    // 型強制によってs1がString型から&mut String型へ変換され、
    // &s2は&String型から&str型へ変換される
    s1.push_str(&s2);

    // 型強制がないとこう書く必要がある
    // (&mut s1).push_str(s2.as_str());

    

    // ポインタの弱体化
    // ・可変性の除去 &mut T型から&T型へ, *mut T型から*const T型へ
    // ・生ポインタへの変換 &mut T型から*mut T型へ, &T空から*const T型へ
    fn func1(slice: &[usize]) -> usize {
        slice.len()
    }

    fn func2(slice: &mut [usize]) {
        // ポインタの弱体化により、&mut [usize]型から&[usize]型へ型強制される
        let len = func1(slice);
        slice[0] = len;
    }

    let mut v = vec![0; 10];
    func2(&mut v[..]);
    assert_eq!(10, v[0]);


}