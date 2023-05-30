pub fn run() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);
    // 所有権がmoveしているので下記はs1でエラー
    // println!("{} {}", s1, s2);
    // moveが発生するのはString型, ベクタ型, Boxポインタなど。所有権の移動が発生

    let i1 = 1;
    let i2 = i1;
    println!("{} {}", i1, i2);
    println!("Stack address of i1 is: {:p}", &i1);
    println!("Stack address of i2 is: {:p}", &i2);
    // 整数型はCopy traitを実装しているため、このようなときにコピーが渡される。所有権のmoveは発生しない

    let sl1 = "literal";
    let sl2 = sl1;
    println!("{} {}", sl1, sl2);
    println!("Stack address of sl1 is: {:p}", &sl1);
    println!("Stack address of sl2 is: {:p}", &sl2);
    // 文字列スライスは所有権のmoveは発生しない。ptrは静的領域のアドレスを"参照"しているから

    // deepcopy →実データを丸ごとヒープの別領域にコピーして別変数に所有させる
    let s3 = String::from("hello");
    let s4 = s3.clone();  // ここでdeepcopy
    println!("Stack address of s3 is: {:p}", &s3);
    println!("Stack address of s4 is: {:p}", &s4);
    println!("Heap memory address of hello: {:?}", s3.as_ptr());
    println!("Heap memory address of hello: {:?}", s4.as_ptr());
    println!("{} {}", s3, s4);

    let s5 = String::from("hello");
    println!("Stack address of s5: {:p}", &s5);
    println!("Heap address of hello: {:?}", s5.as_ptr());
    println!("Len is: {}", s5.len());
    println!("Cap is: {}", s5.capacity());
    take_ownership(s5);
    // s5の所有権は関数の引数に渡した時点で、関数ないにmoveしているため、下記はエラー
    // println!("{}",s5);

    let s6 = String::from("hello");
    println!("Stack address of s6: {:p}", &s6);
    println!("Heap memory address of hello: {:?}", s6.as_ptr());
    println!("Len of s6: {}", s6.len());
    // ここで関数にs6の値の所有権が移り、戻り値sの所有権はs7に引き継がれる
    let s7 = take_giveback_ownership(s6);
    println!("Stack address of s7: {:p}", &s7);
    println!("Heap memory address of hello: {:?}", s7.as_ptr());
    println!("Len of s7: {}", s7.len());
    // s6とs7の実データのヒープアドレスが同じである事が確認できる

    // ただし、実務的には関数に参照を渡す方がやりやすい

    let s8 = String::from("hello");
    let len = calculate_length(&s8);
    println!("The length of '{}' is {}.", s8, len);

    let mut s9 = String::from("hello");
    change(&mut s9);  // &mutでミュータブルな変数を渡す→関数内で変更されている
    println!("{}", s9);

    //  immutableな参照と、mutableな参照の違い→immutableな参照は複数作成できる
    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{} {} {}", s10, r1, r2);

    //  immutableな参照と、mutableな参照は共存できない
    // let mut s10 = String::from("hello");
    // let r1 = &s10;
    // ここでエラーとなる
    // let r2 = &mut s10;
    // println!("{} {}", r1, r2);

    // mutableな参照が有効なスコープでは、所有者も値を読み出せない
    let mut s11 = String::from("hello");
    let r1 = &mut s11;
    // println!("{}", s11); s11のmutableな参照を持つr1が使われるまで、s11は値にアクセスできないため左はエラー
    println!("{}", r1);
    println!("{}", s11);

    let mut s12 = String::from("hello");
    let r1 = &s12;  // immutableな参照
    let r2 = &s12;  // immutableな参照
    println!("{} and {}", r1, r2);  // r1, r2をここで使うと
    let r3 = &mut s12;  // ここでs12のmutableな参照を使える
    *r3 = String::from("hello_updated");
    println!("{}", s12);

    // Memory concept of reference
    // 参照と借用
    let s1 = String::from("hello");
    let sr1 = &s1;  // immutableな参照
    let sr2 = &s1;  // immutableな参照
    println!("Stack address of s1: {:p}", &s1);  // s1のstack address
    println!("Heap memory address of hello: {:?}", s1.as_ptr());
    println!("Len is: {}", s1.len());
    println!("Capacity is: {}", s1.capacity());
    println!("Value of reference sr1: {:p}", sr1);    // s1のstack addressがsr1のptrに格納されている
    println!("Value of reference sr2: {:p}", sr2);    // s1のstack addressがsr2のptrに格納されている
    println!("Stack address of sr1: {:p}", &sr1);
    println!("Stack address of sr2: {:p}", &sr2);
}

fn take_ownership(s: String) {
    println!("Stack address of s: {:p}", &s);
    println!("Heap address of s: {:?}", s.as_ptr());
    println!("Len of s: {}", s.len());
    println!("Cap is: {}", s.capacity());
    println!("{}", s);
}

fn take_giveback_ownership(s: String) -> String {
    s // 関数内の最後の行に;がない場合はその行の値をリターンするという意味
}

// String型の参照を受け取る関数
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("_world");
}

/*
セクション15動画冒頭
参照と借用の整理

参照: &を使ってアドレスを渡すこと。→所有権が適用される型(String型。ベクタ型などデータを保有する)+プリミティブ型(コピートレイトを保有。スタック上で管理される。bool。配列。float)
借用: 所有権を移動させずに参照のみ。プリミティブ型には所有権がないので借用もない。関数の引数で参照する場合などは関数引数には所有権は移動しない
 */