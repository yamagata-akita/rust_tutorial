// タプル構造体
// フィールドに名前を与えず、0から始まる連番のフィールド名を用いる

struct Triangle(Vertex, Vertex, Vertex);
struct Vertex(i32, i32);

// デザインパターン「newtype」
// 「型エイリアスの代わり」にフィールドが1つのタプル構造体を定義することで
// コンパイラの型チェックを強化するテクニック
struct UserName(String);
struct Id(u64);
struct Timestamp(u64);
type User = (Id, UserName, Timestamp);

fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
    (id, name, created)
}


fn main() {
    let vx0 = Vertex(0, 0);
    let vx1 = Vertex(3, 0);
    let triangle = Triangle(vx0, vx1, Vertex(2, 2));

    assert_eq!((triangle.1.0), 3);

    // newtype
    let id = Id(400);
    let now = Timestamp(4567890123);

    // nowとidの順番を間違えるとコンパイルエラーになってくれる
    // let bad_user = new_user(UserName(String::from("kazuki")), now, id);
    // →mismatched types
    // 正しくは
    let  correct_user = new_user(UserName(String::from("kazuki")), id, now);
}