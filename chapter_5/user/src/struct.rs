// 構造体 デフォルト値の設定:
// 構造体に対してDefaultトレイト(std::default::Default)を実装すると
// フィールドのデフォルト値が設定できる
// deriveアトリビュートで自動導出
// すべてのフィールドがデフォルト値を持つPolygonを作成する

struct Polygon {
    vertexes: Vec<(i32, i32)>,
    stroke_width: u8,
    fill: (u8, u8, u8),
}

// Defaultを自動導出するには、構造体のすべてのフィールドの型がDefaultトレイトを実装している必要がある
// implブロックでDefaultトレイトを実装する
impl Default for Polygon {
    fn default() -> Self {
        Self {
            stroke_width: 1,
            vertexes: Default::default(),   // Vec<i32, i32>のDefault実装を使う
            fill: Default::default(),       // (u8, u8, u8)のDefault実装を使う
        }
    }
}

fn main() {
    let polygon1: Polygon = Default::default();
    
    // vertexesフィールドだけ別の値に設定し、他はデフォルト値にする
    let polygon2 = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        .. Default::default()
    };
}