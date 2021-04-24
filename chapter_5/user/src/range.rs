fn main() {
    // 範囲(std::ops::Range)
    // 範囲はstart..end, start..=end, start.., ..end
    // などの形を取り、数列の作成やスライスの範囲指定などで使われる
    
    let a = ['a', 'b', 'c', 'd', 'e'];

    // 糖衣構文と実際の範囲の対応
    assert_eq!(a[ .. ], ['a', 'b', 'c', 'd', 'e']);
    assert_eq!(a[ .. 3], ['a', 'b', 'c']);
    assert_eq!(a[ ..=3], ['a', 'b', 'c', 'd']);
    assert_eq!(a[1..], ['b', 'c', 'd', 'e']);
    assert_eq!(a[1..3], ['b', 'c']);

    // 糖衣構文とRange*型の対応
    assert_eq!(  ..  , std::ops::RangeFull);
    assert_eq!(  .. 3, std::ops::RangeTo {end: 3});
    assert_eq!(  ..=3, std::ops::RangeToInclusive {end: 3});
    assert_eq!(1 ..  , std::ops::RangeFrom {start: 1});
    assert_eq!(1 .. 3, std::ops::Range {start: 1, end: 3});
    assert_eq!(1 ..=3, std::ops::RangeInclusive::new(1, 3));

    // オプション型(std::option::Option<T>)
    // 値があるかどうかわからないことを表す型
    // 列挙型として定義されており、Some(T型の値)とNoneの2つのバリアントを持つ
    // T型の値があるときはSomeで包み、ないときはNoneを使う

    let a1 = ['a', 'b', 'c', 'd'];
    assert_eq!(a1.get(0), Some(&'a'));      // インデックス0は範囲内なのでSome(&値)が返る
    assert_eq!(a1.get(4), None);            // 範囲外なのでNoneが返る

    let mut o1 = Some(10);                  // Option<i32>型
    match o1 {                              // match式でバリアントが判別できる
        Some(s) => assert_eq!(s, 10),
        None => unreachable!(),
    }

    o1 = Some(20);
    if let Some(s) = o1 {                   // if let式でもバリアントの判別と値の取り出しができる
        assert_eq!(s, 20);
    }

    // パターンマッチによってSome(値)をアンラップ(開封)し、中の値が取り出せる
    let mut o2 = Some(String::from("Hello"));   // Option<String>型
    assert_eq!(o2.unwrap(), "Hello");           // unwrap()でSomeの中の値が取り出せる

    // しかしunwrap()はNoneのときにpanicするので、できるだけ使わないほうが良い
    o2 =None;
    // o2.unwrap();

    // unwrap_or_else()ならNoneでもパニックしないので、安心して使える
    // Noneのときはクロージャを実行し、Noneの代わりになる値を得る
    assert_eq!(o2.unwrap_or_else(|| String::from("o2 is none")), "o2 is none");

    // Someで包まれた値を操作するならmap()やand_then()などのコンビネータが便利

    // map()はSome(値)のときは値にクロージャを適用し、クロージャが返した値をSomeで包みなおす
    let mut o3 = Some(25);
    assert_eq!(o3.map(|n| n * 10), Some(250));

    // NoneならなにもせずNoneを返す
    o3 = None;
    assert_eq!(o3.map(|n| n * 10), None);

    o3 = Some(10);
    assert_eq!(
        o3.map(|n| n * 10)
            // and_then()はSome(値)のときは値にクロージャを適用し
            // クロージャが返した値(Some(新しい値), またはNone)をそのまま返す
            .and_then(|n| if n >= 200 { Some(n) } else { None }),
        None
    );

    // 複数のオプション値がすべてSomeのときに処理を実行したい場合、?演算子が便利
    fn add_elems(s: &[i32]) -> Option<i32> {
        // Some(値)なら値を取り出し、Noneならこの関数からすぐに戻る(Noneを返す)
        let s0 = s.get(0)?;
        let s3 = s.get(3)?;
        Some(s0 + s3)
    }

    assert_eq!(add_elems(&[3, 7, 31, 127]), Some(3 + 127));
}