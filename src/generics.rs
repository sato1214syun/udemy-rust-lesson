// 構造体
struct Point<T> {
    x: T,
    y: T,
}

// x, yで違う型を入れてもいいようにするにはジェネリクスを2つつける
struct PointAnother<T, U> {
    x: T,
    y: U,
}

// 構造体に関数を実装
impl<T, U> PointAnother<T, U> {
    fn mixup<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
        PointAnother {
            x: self.x,
            y: other.y,
        }
    }
}
pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];
    // let mut largest = number_list[0];
    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("The largest is {}", largest);
    // println!("{}", largest_i32(number_list));
    let char_list = vec!['a', 'b', 'c', 'd']; // ""で囲うと文字列リテラル。''だとchar型(4byte)
    println!("{}", largest(char_list));
    println!("{}", largest(number_list));
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.0, y: 2.0 };
    let p3 = PointAnother { x: 5, y: 10.4 };
    let p4 = PointAnother { x: "Rust", y: 'a' };
    let p5 = p3.mixup(p4);
    println!("{} {}", p5.x, p5.y);
}
fn largest_i32(list: Vec<i32>) -> i32 {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// ジェネリクス
fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {  // 比較が必要なので トレイト境界としてPartialOrdが必要また、Copyを設定
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
