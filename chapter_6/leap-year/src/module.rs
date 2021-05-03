// モジュールとアイテムの可視性
// プログラムを複数のモジュールに分割するには、modキーワードを使う


// モジュールの中のアイテムは、指定しない限りはモジュールの内側でしかアクセスできない
// プライベートなアイテムとして作成される

// pub fn 関数(){}
// どこからでもアクセスできる、完全にパブリックなアイテム

// pub(crate) fn 関数(){}
// モジュールの含まれるクレートに対してはパブリックな関数

// pub(in app::network) fn 関数(){}
// app::networkモジュールに対してはパブリックな関数



mod server {
    // この中はserverモジュール
    pub fn echo() {
        // これはserver::echo()関数
        println!("Server");
    }
}

mod client {
    // この中はclientモジュール
    pub fn echo() {
        // これはclient::echo()関数
        println!("Client");
    }    
}

fn main() {
    // serverモジュールの中のecho()関数の呼び出し
    server::echo();

    // clientモジュールの中のecho()関数の呼び出し
    client::echo();
}