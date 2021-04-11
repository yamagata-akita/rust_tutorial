fn main() {
    println!(
        // {:.1}で小数点以下1桁まで表示
        "半径 {:.1}, 円周率 {:.3}, 面積{:.3}",
        3.2,
        std::f64::consts::PI,
        3.2f64.powi(2) * std::f64::consts::PI,
    );
}
