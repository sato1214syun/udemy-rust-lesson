/*
Rustのメモリ安全性
二重解放エラー→所有権システムで保証される
メモリ解放忘れ→ RAII (Resource acquisition Is Initializing)変数初期化時にリソース(メモリ)が確保される。変数がスコープを抜けるときにリソースが解放される。
ダングリングポインター→ライフタイム
 */

pub fn run() {
    let st1 = String::from("x");
    let st2 = String::from("y");
    let res1 = get_longest(&st1, &st2);
    println!("{}", res1);

    let st3 = String::from("x");
    let res2;
    {
        let st4 = String::from("y");
        res2 = get_longest(&st3, &st4);
        println!("{}", res2);
    }

}

// ジェネリックライフタイムアノーテーション<'a>の所
// <'a>, &'aをつけないと、返り値のライフタイムに引数x,yのライフタイムのどちらを適用すればいいかわからないとエラーが出る
// こう書くと、引数で受け取った変数のライフタイムの短い方を返り値に適用するという指示になる
fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str { // 文字の大小を比較して大きい方を返す
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// fn dummy1<'a>() -> &'a str {
//     let s = String::from("demo");
//     &s  // 関数ローカル変数の参照は返せない。ダングリングポインタを引き起こしてしまう
// }
// fn dummy2<'a>() -> &'a i32 {
//     let x = 10;
//     &x  // dummy1と同じようにエラー
// }
fn dummy3() -> String {
    let s = String::from("demo");
    s
}
