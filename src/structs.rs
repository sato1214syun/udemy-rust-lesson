#[derive(Debug)]  // デバッグトレイトを実装
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Rectangleにメソッドを追加
impl Rectangle {
    // 幅と高さを受け取って、新たに幅、高さの構造体を作る
    fn create(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    // 面積を計算して表示 &selfはインスタンスの参照を受け取ることを示す。
    // &がない場合は元のインスタンスの所有権が移動するので、元のインスタンスが使えなくなる
    fn area(&self) {
        println!("{}", self.width * self.height);
    }
}

pub fn run() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // let user2 = user1;  // 所有権が移動する

    // immutableな変数を作成。後から数値尾を変更できる
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("{:#?}", user1);  // 構造体の先頭に#[derive(Debug)]をつけていないと構造体がデバッグできないためここでエラーが出る。また、{#}をつけておくと出力がフォーマットされる

    let user2 = build_user(String::from("user2@xxx.com"), String::from("user2"));
    println!("{:#?}", user2);

    let rect = Rectangle::create(20, 20);
    println!("{:#?}", rect);
    rect.area();
    println!("{:#?}", rect); // areaの引数がself(&なし)の場合に所有権がなくなるので、エラーになる
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
