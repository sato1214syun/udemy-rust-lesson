pub mod sub_a;
pub mod sub_b;

const MAX_POINTS: u32 = 100_000;

pub fn run() {
    println!("Here is vars module!!");
    // sub_a::func_a();
    // sub_b::func_b();
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let _i1 = 3;
    let _f1 = 0.1;

    println!("{}", usize::BITS);
    println!("Memory address of const is: {:p}", &MAX_POINTS);

    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("Stack address of i2 is: {:p}", &i2);
    println!("Stack address of i3 is: {:p}", &i3);

    let y = 5;
    println!("Stack address of y is: {:p}", &y);
    let y = y + 1;
    println!("Stack address of y is: {:p}", &y);
    let y = y * 2;
    println!("Stack address of y is: {:p}", &y);
    println!("The value of y is: {}", y);
    {
        let y = 0;
        println!("The value of y is :{}", y);
    }
    println!("The value of y is: {}", y);

    // タプル
    let t1 = (500, 6.4, "dummy");
    let (_x, _y, _z) = t1;
    println!("The value of t1 is: {} {} {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    *x1_ptr = 5;
    *y1_ptr = -5;
    println!("{:?}", t2);

    // 配列
    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]);

    // 文字列スライス
    let s1 = "helloこんにちは挨拶"; //26bytes utf-8ではアルファベット1バイト/文字, 日本語3バイト/文字
    let s2 = "hello";
    // スタック領域のアドレス
    println!("Stack address of s1 is: {:p}", &s1);  // 16byte差→1つの文字列スライス型変数で16byte占める(固定)
    println!("Stack address of s2 is: {:p}", &s2);  // ptr(pointer)で8, len(length)で8byte使っている
    // 静的メモリ領域のアドレス
    println!("Static memory address of s1: {:?}", s1.as_ptr());  // 26byte差→実データ
    println!("Static memory address of s2: {:?}", s2.as_ptr());
    // バイト長
    println!("Len of s1 is: {}", s1.len());  // 26
    println!("Len of s2 is: {}", s2.len());  // 5

    // String型
    let mut s1 = String::from("hello");
    let mut s2 = String::from("hello world");
    // スタック領域のアドレス
    println!("Stack address of s1 is: {:p}", &s1);  // 24byte差→1つのストリング型変数で24byte占める(固定)
    println!("Stack address of s2 is: {:p}", &s2);  // ptr(pointer)で8, len(length)で8, cap(capacity)で8byte使っている
    // ヒープ領域のアドレス
    println!("Heap memory address of s1: {:?}", s1.as_ptr());  // 16byte差
    println!("Heap memory address of s2: {:?}", s2.as_ptr());
    // バイト長
    println!("Len of s1 is: {}", s1.len());
    println!("Len of s2 is: {}", s2.len());
    // キャパシティ→バイト長と同じ数値が格納されている
    println!("Capacity of s1 is: {}", s1.capacity());
    println!("Capacity of s2 is: {}", s2.capacity());
    // push_strで文字列を追加
    s1.push_str("_new1");
    s2.push_str("_new2");
    println!("{} {}", s1, s2);

    /*所有権
    
    所有権はメモリの二重解放を回避するためにある
    同じヒープ領域を参照する2つの変数があるとき、所有権者(メモリ解放する権限のある変数)が両者であるとき、
    どちらかが領域を解放した後に、アロケーターが開いた領域に別のデータを割り当てる
    もう一方の変数が領域を解放しようとしたときに、この別データを開講してしまい問題が起こる
    所有権者が一つであればこのようなことは起こらない

    文字列スライスのみ、データの所有権が適用されない。参照のみ。
    文字列スライスの作り方は２通りあるがそれぞれ別の理由で所有権がない
    リテラルから直で作成→文字列スライスは静的領域(プログラム実行中永続的に確保されるメモリ領域。グローバル変数などの格納場所)を用いるため、解放する必要がない
    String型から文字列スライスを作成→String型変数から所有権を移動せず参照する権利を文字列リテラルに貸し出している。これを借用という。
    具体的な操作としては 文字列スライス変数: &str = &String型変数 となり、内部的にはtring型変数のptr(ヒープ領域の先頭アドレス)を参照させている。
    */

}
