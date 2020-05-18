use super::SortOrder;

pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    if x.len().is_power_of_two() {
        match *order {
            SortOrder::Ascending  => do_sort(x, true),
            SortOrder::Descending => do_sort(x, false),
        };
        Ok(())
    } else {
        Err(format!("The length of x is not a power of two. (x.len(): {}", x.len()))
    }
}


// 型パラメータTを導入して関数をジェネリクス化する
// ただ単に<T>と宣言すると、Tはあらゆる型を示すようになる(大小比較できない型も)
fn do_sort<T: Ord>(x: &mut [T], up: bool) {
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        assert_eq!(sort(&mut x[..mid_point], &SortOrder::Ascending), Ok(()));
        assert_eq!(sort(&mut x[mid_point..], &SortOrder::Descending), Ok(()));
        sub_sort(x, up);
    }
}


fn sub_sort<T: Ord>(x: &mut [T], up: bool) {
    if x.len() > 1 {
        compare_and_swap(x, up);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], up);
	    sub_sort(&mut x[mid_point..], up);
    }
}


fn compare_and_swap<T: Ord>(x: &mut [T], up: bool) {
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        if (x[i] > x[mid_point + i]) == up {
            x.swap(i, mid_point + i);
        }
    }
}


#[cfg(test)]
mod tests {
    // 親モジュール(first)のsort関数を使用する
    use super::{is_power_of_tow, sort, sort_by};
    use crate::SortOrder::*;

    // 構造体Studentを定義する
    // 構造体は関連する値を1つにまとめたデータ構造。複数のデータフィールドを持つ
    struct Student {
        first_name: String, // first_name(名前)フィールド。String型
        last_name:  String, // last_name(名字)フィールド。 String型
        age: u8,            // age(年齢)フィールド。u8型(8ビット符号なし整数)
    }

    // implブロックを使うと対象の型に関連関数やメソッドを実装できる
    imple Student {

        // 関連関数newを定義する(OOPでいうスタティックメソッド)
        fn new(first_name: &str, last_name: &str, age: u8) -> Self {

            // 構造体Studentを初期化して返す。Selfはimpl対象の型(Student)の別名
            Self {
                // to_stringメソッドで&str型の引数からString型の値を作る
                first_name: first_name.to_string(),
                last_name:  last_name.to_string(),
                age, // ageフィールドにage変数の値を設定
                     // フィールドと変数が同じ名前のときは、このように省略形で書ける
            }
        }
    }

    #[test]
    // 年齢で昇順にソートする
    fn sort_students_by_age_ascending() {

        // 4人分のテストデータを作成
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        // ソート対象のベクタを作成する
        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];

        // ソート後の期待値を作成する
        let expected = vec![&hanako, &kyoko, &taro, &ryosuke];

        assert_eq!(
            // sort_by関数でソートする。第二引数はソート順を決めるクロージャ
            // 引数に2つのStudent構造体をとり、ageフィールドの値をcmpメソッドで比較することで大小を決定する
            // u8型のcmpメソッドは2つのu8型の値を比較してstd::cmp::Ordering型の値を返す
            // Ordering::Less or Ordering::Equal or Ordering:: Greater
            sort_by(&mut x, &|a, b| a.age.cmp(&b.age)),
            Ok(())
        );

        // 結果を検証する
        assert_eq!(x, expected);
    }

    #[test]
    // 氏名で昇順にソートする
    fn sort_students_by_age_ascending() {

        // 4人分のテストデータを作成
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];

        let expected = vec![&ryosuke, &kyoko, &hanako, &taro];

        assert_eq!(sort_by(&mut x,
            // まずはlast_nameを比較する
            &|a, b| a.last_name.cmp(&b.last_name)
                // もしlast_nameが等しくない(LessまたはGreater)ならそれを返す
                // last_nameが等しいなら(Equal)first_nameを比較する
                .then_with(|| a.first_name.cmp(&b.first_name))), Ok(())
        );

        // 結果を検証する
        assert_eq!(x, expected);
    }

    #[test]
    fn sort_u32_ascending() {
        // xに型注釈Vec<u32>をつける
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }


    #[test]
    fn sort_u32_descending() {
        // xに型注釈Vec<u32>をつける
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }


    #[test]
    fn sort_str_ascending() {
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert_eq!(x, vec!["GC", "Rust", "and", "fast", "is", "memory-efficient", "no", "with"]);
    }

    #[test]
    fn sort_str_descending() {
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        assert_eq!(x, vec!["with", "no", "memory-efficient", "is", "fast", "and", "Rust", "GC"]);
    }

    #[test]
    fn sort_to_fail() {
        let mut x = vec![10, 30, 11]; // x.len()が2のべき乗になっていない
        assert!(sort(&mut x, &Ascending).is_err()); // 戻り値はErr
    }
}
