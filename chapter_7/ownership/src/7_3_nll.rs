// 借用規則
// 1.不変、可変を問わず参照のライフタイムが値のスコープより短い
// 2.値が共有されている間(不変の参照が有効な間)は値の変更を許さない

// この規則が守られているかは、コンパイラの借用チェッカ(borrow checker)によって検査される

// 借用チェッカ(NLL: Non-Lexical Lifetime)
// 制御フローグラフに基づくライフタイムの推論を行う

use std::collections::HashMap;

// この関数はHashMapにキーに対応する値がある場合はそれを変更し
// ない場合はデフォルト値を挿入する
fn process_or_default(key: char, map: &mut HashMap<char, String>) {
    // get_mutが返す可変の参照が生存している間は、mapの可変の借用が有効
    match map.get_mut(&key) {
        // valueが可変の参照に束縛される
        // つまりvalueが生存している間はmapの可変の借用が有効となる
        Some(value) => value.push_str(", world!"),
        None => {
            // このブロック内ではselfの可変の借用は終了している
            // insertはselfの可変の借用を取る
            map.insert(key, Default::default());
        }
    }
}

/*
このコードではmapの可変の借用を取るメソッドが2つ使われている
1.get_mut
2.insert
借用規則により、mapに対する可変の借用が2つ存在することは許されないため、
これらの借用のライフタイムが重ならないようにしなければならない

get_mutはOption<&mut String>型を返す。
この戻り値(可変の参照)が生存している間は、mapの可変の借用が有効
Noneでは戻り値に何も束縛していない=get_mutによる可変の借用は終了している
→insertで問題なく可変の借用を取れる
*/


fn main() {
    let mut map = HashMap::new();
    map.insert('h', "Hello".to_string());
    process_or_default('h', &mut map);
}