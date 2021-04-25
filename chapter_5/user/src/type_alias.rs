fn main() {
    // 複合型の定義
    // 構造体(struct)や列挙型(enum)を使う
    // 型エイリアスを使うと、既存の方に別名をつけられる

    // 型エイリアス
    // 型エイリアス = 型につけられる別名で、typeキーワードで定義する

    // 型エイリアスは大文字を使った定義？
    type UserName = String;
    type Id = i64;
    type Timestamp = i64;
    type User = (Id, UserName, Timestamp);

    fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
        (id, name, created)
    }

    let id = 400;
    let now = 4567890123;
    let user = new_user(String::from("mika"), id, now);

    // 型エイリアスは型のネストが深くなったときに使うと便利
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    pub type SharedMap<K, V> = Rc<RefCell<HashMap<K, V>>>;




    // 構造体
    // 構造体(struct)は複数の関連する値を1つにまとめたデータ構造
    // 英語で構造を意味するstructureから来ている
    // 構造体の3つの種類
    // 名前付きフィールド構造体
    // タプル構造体
    // ユニット構造体




    // 名前付きフィールド構造体
    struct Polygon {
        vertexes: Vec<(i32, i32)>,  // 頂点の座標
        stroke_width: u8,           // 輪郭の太さ
        fill: (u8, u8, u8),         // 塗りつぶしのRGB
    }

    // タプル構造体
    struct Triangle(Vertex, Vertex, Vertex);
    struct Vertex(i32, i32);

    // ユニット構造体
    struct UniqueValue;
    // または
    // struct UniqueValue {}
    // struct UniqueValue();



    // 値の初期化
    let triangle = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        stroke_width: 1,
        fill: (255, 255, 255),
    };

    // フィールド名と同じ名前の関数引数やローカル変数があるときは以下のような
    // 省略形で初期化できる
    fn new_polygon(vertexes: Vec<(i32, i32)>) -> Polygon {
        let stroke_width = 1;
        let fill = (0, 0, 0);
        Polygon {vertexes, stroke_width, fill}
    }
    let quadrangle = new_polygon(vec![(5, 2), (4, 7), (10, 6), (8, 1)]);

    // フィールドへのアクセス
    // 取り出しや書換の方法
    // フィールド名を使うか、パターンマッチで分解するか

    // フィールド名でアクセス
    assert_eq!(triangle.vertexes[0], (0, 0));
    assert_eq!(triangle.vertexes.len(), 3);
    assert_eq!(triangle.fill, (0, 0, 0));

    // パターンマッチでアクセス。不要なフィールドは..で省略できる
    let Polygon {vertexes: quad_vx, ..} = quadrangle;
    assert_eq!(4, quad_vx.len());

    // :以降を省略すると、フィールドと同じ名前の変数が作られフィールド値に束縛される
    let Polygon {fill, ..} = quadrangle;
    assert_eq!((0, 0, 0), fill);

    // 構造体の値を変更するにはmutが必要
    let mut polygon = new_polygon(vec![(-1, -5), (-4, 0)]);
    assert_eq!(polygon.vertexes.len(), 2);
    polygon.vertexes.push((2, 8));
    assert_eq!(polygon.vertexes.len(), 3);

    // すでにある値をもとにして、その一部を使った新しい値を作れる
    let triangle1 = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        fill: (255, 255, 255),
        stroke_width: 5,
    };

    // triangle1をもとにvertexesだけ異なる新しい値を作る
    // この構文は関数型レコードアップデートと呼ばれる
    let triangle2 = Polygon {
        vertexes: vec![(0, 0), (-3, 0), (-2, 2)],
        .. triangle1
    };

    // デフォルト値の設定
    // let bad_polygon = Polygon {
    //     vertexes: vec![(0, 0), (3, 0), (2, 2)],
    // };
    // →コンパイルエラー(missing fields 'fill', 'stroke_width' in initializer of 'Polygon)
    
    // 構造体に対してDefaultトレイト(std::default::Default)を実装するとフィールドのデフォルト値が設定できる
    // deriveアトリビュートで自動導出できる
    // struct.rsへ続く
    /*
    #[derive(Default)]
    struct Polygon {
        vertexes: Vec<(i32, i32)>,
        stroke_width: u8,
        fill: (u8, u8, u8),
    }
    */




}