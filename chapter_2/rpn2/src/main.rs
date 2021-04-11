fn main() {
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";

    let ans = rpn(exp);

    // デバッグビルド時のみ、答えが正しいかチェックする
    // 浮動小数点の計算誤差を考慮し、ここでは小数点以下4桁までの値を文字列に変換する
    debug_assert_eq!("26.2840", format!("{:.4}", ans));

    println!("{} = {:.4}", exp, ans);
}


fn rpn(exp: &str) -> f64 {
    // stackはミュータブルな変数で、値の変更を許す
    let mut stack = Vec::new();

    for token in exp.split_whitespace() {
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        }
        else {
            match token {
                "+" => apply2(&mut stack, |x, y| x + y),
                "-" => apply2(&mut stack, |x, y| x - y),
                "*" => apply2(&mut stack, |x, y| x * y),
                "/" => apply2(&mut stack, |x, y| x / y),

                // tokenが演算子でないなら、エラーを起こして終了
                _ => panic!("Unknown operator: {}", token),
            }
        }
    }
    stack.pop().expect("Stack underflow")
}

fn apply2<F>(stack: &mut Vec<f64>, fun: F)
where
    F: Fn(f64, f64) -> f64,
{
    // 変数y,xをスタックの最後の2要素に束縛する
    if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
        // クロージャfunで計算し、その結果を変数zに束縛する
        let z = fun(x, y);
        stack.push(z);
    } else {
        // スタックから要素が取り出せなかったときはエラーを起こして終了
        panic!("Stack underflow");
    }
}