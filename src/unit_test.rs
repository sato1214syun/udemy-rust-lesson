//  unit testの仕方
// 構造体Rectangleを定義して、compare_areaで他のRectangleと面積を比較できるようにする
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn compare_area(&self, other: &Rectangle) -> bool {
        self.width * self.height > other.width * other.height
    }
}

// 整数を2倍
fn double_value(a: i32) -> i32 {
    a * 2
}

// 挨拶を返す
fn greeting(name: &str) -> String {
    format!("Hello {} san", name)
}

// 以下test構文
#[cfg(test)]  // cargo testを実行した時のみ以下が実行される
mod tests {
    // 上記の関数、構造体の階層より1個下の階層になるため、呼び出しにuse superが必要
    use super::*;

    #[test]
    fn test_a_is_larger() {
        let a = Rectangle {
            width: 5,
            height: 5,
        };
        let b = Rectangle {
            width: 3,
            height: 3,
        };
        assert!(a.compare_area(&b));
    }

    #[test]
    fn test_a_is_smaller() {
        let a = Rectangle {
            width: 3,
            height: 3,
        };
        let b = Rectangle {
            width: 5,
            height: 5,
        };
        assert!(!(a.compare_area(&b)));
    }

    #[test]
    fn test_double() {
        assert_eq!(6, double_value(3));
    }

    #[test]
    fn test_contains_name() {
        let res = greeting("rust");
        assert!(res.contains("rust"));
    }
}
