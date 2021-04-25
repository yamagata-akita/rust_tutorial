// 列挙型(enum)
// 異なる種類の値を1つにまとめた型を定義する
// 例: Option<T>型
// Option<T>型はSome(T)とNoneの2つの異なる種類の値を持つ

// C言語風の列挙型
// 平日を表すWeekday型
// Debugトレイトを自動導出すると"{:?}"で表示できるようになる
// PartialEqトレイトを自動導出すると==演算子が使えるようになる

#[derive(Debug, PartialEq)]
enum Weekday {
    // weekday型には以下のバリアントがある。
    Monday, Tuesday, Wednesday, Thursday, Friday,
}


// データを持たない列挙型では個々のバリアントにisize型の整数値を割り当てられる
enum Month {
    January = 1, February = 2, March = 3, April = 4, May = 5, June = 6,
    July = 7, August = 8, September = 9, October = 10, November = 11, December = 12,
}

// データをもつ列挙型
// バリアントには構造体と同じ文法でフィールドをもたせられる
type UserName = String;
#[derive(Debug)]
enum Task {
    Open,
    AssignedTo(UserName), // タプル構造体
    Working {
        assignee: UserName,
        remaining_hours: u16,
    }, // 名前付きフィールド構造体
    Done,
}


fn say_something(weekday: Weekday) {
    if weekday == Weekday::Friday {
        println!("TGIF!");          // Thank God, it's Friday(やっと金曜日だ)
    } else {
        println!("まだ{:?}か", weekday);
    }
}

fn main() {
    say_something(Weekday::Friday);
    assert_eq!(3, Month::March as isize);   // isize型にキャストすると割り当てた値が得られる

    // use宣言でTaskが持つバリアントをインポートすると、バリアント名が直接書けるようになる
    use crate::Task::*;

    // Task型の値を3つ作り、ベクタに格納する
    // tasksベクタに格納されている値はすべてTask型
    let tasks = vec![
        AssignedTo(String::from("junko")),
        Working {
            assignee: String::from("hiro"),
            remaining_hours: 18,
        },
        Done,
    ];

    // 個々のタスクの状況をレポートする
    for (i, task) in tasks.iter().enumerate() {
        // match式によるパターンマッチでバリアントを識別子、フィールド値を取り出す
        match task {
            AssignedTo(assignee) => {
                println!("タスク{}は{}さんにアサインされています", i, assignee);
            }
            Working {assignee, remaining_hours} => {
                println!("タスク{}は{}さんが作業中です。残り{}時間の見込み", i, assignee, remaining_hours);
            }
            _ => println!("タスク{}はその他のステータス({:?})です", i, task)
        }
    }




}