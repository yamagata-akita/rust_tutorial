// staticライフタイム
// プログラム終了時まで続き、ほかのどのライフタイムよりも長く生存する
// staticライフタイムを持つ参照は、基本的にstatic変数の値などのコンパイル時に値が確定するもの
// からしか作られない

fn main() {
    static IO: i32 = 42;        // static変数。'staticスコープを持つ

    let mut s0: &'static str;
    let s1 = "42";              // &str型。文字列リテラルへの参照(データは静的領域になる)
    let s2 = 42.to_string();    // &String型(データはヒープ領域にある)
    s0 = s1;                    // 文字列リテラルへの参照は'staticライフタイムを持つ
    // s0 = &s2;                // コンパイルエラー。String型から&'static strは作れない
    // ->error 's2' does not live long enough

    // この関数は'staticライフタイムを持つ任意の方を引数に取る
    fn take_static<T: 'static>(_x: T) {}

    let s3 = "42";              // &'static str型
    let s4 = 42.to_string();    // String型

    take_static(s3);
    // take_static(&s4);        // &String型。コンパイルエラー
    take_static(s4);

}