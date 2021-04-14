use super::SortOrder;
use rayon;
use std::cmp::Ordering;

pub fn sort<T: Ord + Send>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    // do_sortを呼ぶ代わりに、sort_byを呼ぶようにする
    // is_power_of_twoはsort_byが呼ぶだめ、ここからは削除
    if x.len().is_power_of_two() {
        match *order {
            SortOrder::Ascending => sort_by(x, &|a, b| a.cmp(b)),
            SortOrder::Descending => sort_by(x, &|a, b| b.cmp(a)),
        };
        Ok(())
    } else {
        Err(format!("The length of x is not a power of two. (x.len(): {})", x.len()))
    }
}

// 並列に処理するか決めるしきい値
const PARALLEL_THRESHOLD: usize = 4096;

fn do_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
    where T: Send,
          F: Sync + Fn(&T, &T) -> Ordering
{    
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        // xを事前に分割しておく(これもだめ)
        // let first = &mut x[..mid_point];
        // let second = &mut x[mid_point..];

        // xをmid_pointを境にした2つの可変の借用に分割し、firstとsecondに束縛する
        let (first, second) = x.split_at_mut(mid_point);

        // xの分割後の要素数をしきい値と比較する
        if mid_point >= PARALLEL_THRESHOLD {
            // しきい値以上なら並列にソートする(並列処理)
            rayon::join(|| do_sort(first, true, comparator),
                        || do_sort(second, false, comparator));
        } else {
            // しきい値未満なら順番にソートする(順次処理)
            do_sort(&mut x[..mid_point], true, comparator);
            do_sort(&mut x[mid_point..], false, comparator);
        }
        sub_sort(x, forward, comparator);
    }
}

fn sub_sort<T, F>(x: &mut [T], forward: bool, comparator: &F)
    where T: Send,
          F: Sync + Fn(&T, &T) -> Ordering
{
    if x.len() > 1 {
        compare_and_swap(x, forward, comparator);
        let mid_point = x.len() / 2;
        let (first, second) = x.split_at_mut(mid_point);
        if mid_point >= PARALLEL_THRESHOLD {
            rayon::join(|| sub_sort(first, forward, comparator),
                        || sub_sort(second, forward, comparator));
        }
    }
}

fn compare_and_swap<T, F>(x: &mut [T], forward: bool, comparator: &F)
    where F: Fn(&T, &T) -> Ordering
{
    // 比較に先立ちforward(bool)をOrdering値に変換しておく
    let swap_condition = if forward {
        Ordering::Greater
    } else {
        Ordering::Less
    };
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        // comparatorクロージャで2要素を比較し、返されたOrderingのバリアントが
        // swap_conditionと等しいなら要素を交換する
        if comparator(&x[i], &x[mid_point + i]) == swap_condition {
            x.swap(i, mid_point + i);
        }
    }
}

pub fn sort_by<T, F>(x: &mut [T], comparator: &F) -> Result<(), String>
    where T: Send,
          F: Sync + Fn(&T, &T) -> Ordering
{
    if x.len().is_power_of_two() {
        do_sort(x, true, comparator);
        Ok(())
    } else {
        Err(format!("The length of x is not a power of two. (x.len(): {})", x.len()))
    }
}


// このモジュールはcargo testを実行したときのみコンパイルされる
#[cfg(test)]
mod tests {
    use super::{sort, sort_by};
    use crate::SortOrder::*;
    use crate::utils::{new_u32_vec, is_sorted_ascending, is_sorted_descending};

    // 構造体Studentを定義する
    // deriveアトリビュートを使い、DebugトレイととPartialEqトレイとの実装を自動導出する
    #[derive(Debug, PartialEq)]
    struct Student {
        first_name: String,
        last_name: String,
        age: u8,
    }

    // implブロックを使うと対象の型に関連関数やメソッドを実装できる
    impl Student {

        // 関連関数newを定義する
        fn new(first_name: &str, last_name: &str, age: u8) -> Self {

            Self {
                // 構造体Studentを初期化して返す。Selfはimpl対象の型(Student)の別名
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
                age, // フィールドと変数が同じ名前のときは、このように省略することができる
            }
        }
    }

    #[test]
    // 年齢で昇順にソートする
    fn sort_students_by_age_ascending() {

        // テストデータを作成
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
            sort_by(&mut x, &|a, b| a.age.cmp(&b.age)),
            Ok(())
        );

        // 結果の検証
        assert_eq!(x, expected);
    }

    #[test]
    // 名前で昇順にソートする
    fn sort_students_by_name_ascending() {

        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];

        let expected = vec![&ryosuke, &kyoko, &hanako, &taro];

        assert_eq!(
            sort_by(&mut x,
                // まずlast_nameを比較する
                &|a, b| a.last_name.cmp(&b.last_name)
                    // もしlast_nameが等しくない(Less, Greater)なら、それを返す
                    // last_nameが等しい(Equal)なら、first_nameを比較する
                    .then_with(|| a.first_name.cmp(&b.first_name)
                )
            ),
            Ok(())
        );

        // 結果の検証
        assert_eq!(x, expected);
    }

    #[test]
    fn sort_u32_large() {
        {
            // 乱数で65536要素のデータ列を作る
            let mut x = new_u32_vec(65536);
            // 昇順にソートする
            assert_eq!(sort(&mut x, &Ascending), Ok(()));
            // ソート結果が正しいことを検証する
            assert!(is_sorted_ascending(&x));
        }
        {
            let mut x = new_u32_vec(65536);
            assert_eq!(sort(&mut x, &Descending), Ok(()));
            assert!(is_sorted_descending(&x));
            
        }
    }
}
