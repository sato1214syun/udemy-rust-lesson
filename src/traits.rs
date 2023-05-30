// トレイト:複数の型のデータに対して、共通の機能を持たせたいときに使う

trait Fruits {
    // 具体的な処理を書かない。下記のrun関数の中で型ごとに処理を記述する
    fn price(&self) -> u32;
}

struct Apple;
// Fruitsトレイトの実装のしかた
impl Fruits for Apple {
    fn price(&self) -> u32 {
        10
    }
}

struct Banana;
// データ型(構造体)Bananaに対して、Fruitsトレイトを適用
impl Fruits for Banana {
    fn price(&self) -> u32 {
        5
    }
}

trait Summary {
    fn summarize(&self) -> String {
        // トレイト内の関数に具体的な処理がある場合は、
        // それはデフォルトの処理として扱われる
        // つまり、impl時のsummarizeに処理を記述しない場合にこの処理が適用される
        String::from("(Read more...)")
    }
}

trait Message {
    fn message(&self) -> String {
        String::from("Message")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}
//
impl Message for NewsArticle {}
impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     // format!は受け取ったデータをString型で返してくれるマクロ
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}
// impl Message for NewsArticle {}
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }`
}
pub fn run() {
    let apple = Apple {};
    let banana = Banana {};
    get_price(apple);
    get_price(banana);
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("{}", article.summarize());
    notify(&article);
    notify_another(&article);
    // notify_another(&tweet);
}

// priceを出力するための関数
// ジェネリクスのトレイト境界でFruitsのみを受け取れるようにしておく<T: Fruits>
fn get_price<T: Fruits>(fruits: T) {
    println!("price is: {}", fruits.price());
}

// 型に&impl Summeryとしているので、Summeryのトレイトを持つ引数を取る
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Summery + Messageの両方のトレイトが実装されているデータを引数にとる
fn notify_another(item: &(impl Summary + Message)) {
    println!("Breaking news! {}", item.summarize());
    println!("Message! {}", item.message());
}
