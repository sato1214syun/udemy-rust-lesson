/*
enum 再帰的に構成されたリスト(列挙型))
*/
enum List {
    // 型にをBox<T>(<>はジェネリクス)をつけるとエラーが消せる→Boxは8バイトで確定するので
    Node(i32, Box<List>),
    Nil,
}

pub fn run() {

    // Rustのスタックの条件は8Mbまでなので、それを超えるとスタックオーバーフローする
    // macではzsh: segmentation faultと表示される
    // let _a1: [u8; 9_000_000] = [1; 9_000_000];
    // ベクタ型 配列の要素を動的に変更したい場合に使う
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];
    // スタック領域のptr
    println!("Stack address of v1 is: {:p}", &v1); //24byte差 ptr, len(要素数), cap 8byteづつ
    println!("Stack address of v2 is: {:p}", &v2);
    // ヒープ領域のじ実データの先頭アドレス
    println!("Heap memory address of v1: {:?}", v1.as_ptr());
    // 要素数、capacity
    println!("Len of v1 is: {}", v1.len());
    println!("Capacity of v1 is: {}", v1.capacity());
    // ベクタ型への要素の追加
    v1.insert(1, 10);
    println!("{:?}", v1);
    v1.remove(0);
    println!("{:?}", v1);
    v1.append(&mut v3);
    println!("{:?}", v1);
    println!("{:?}", v3);

    /*ベクタ型のスタック領域の情報ptrには実データの先頭アドレスと、要素の型の情報(1要素何byte確保されているかも含めて)の両方が格納されている
    なので、先頭アドレスと要素数が分かれば、1要素ずつ辿っていくことが可能
     */

    /*ボックスポインタ
    スタックに存在するデータをヒープに移動させ、ヒープに移動させたデータの先頭アドレスをボックスポインタとしてスタックに保有する
    作成時にサイズの決まらない変数を扱う

    下記tupleで説明するとスタックには、
    */
    let t1: (i64, String) = (10, String::from("hello"));
    println!("Stack address of tuple data is: {:p}", &t1); // tuple t1.0 数値"10" のスタックデータの先頭アドレス
    println!("Heap memory address of t1.1: {:?}", t1.1.as_ptr());  // 実データhelloの先頭アドレス
    println!("Len of t1.1 is: {}", t1.1.len());  // hello分
    println!("Capacity of t1.1 is: {}", t1.1.capacity());  // hello分のキャパシティ(そのベクタのために確保されたヒープ領域の容量)これを超えると、要素が再割り当てされる
    let mut b1 = Box::new(t1);  // ボックスポインタでスタックデータをヒープに対退避、スタック(ポインタb1)には対比したデータの先頭アドレスが格納される
    (*b1).1 += "world";  // 参照はずし*(参照はずし演算子)をして実データを直接編集
    println!("{} {}", b1.0, b1.1);
    println!("Stack address of box pointer is: {:p}", &b1);  // &をつけてスタックアドレスを参照。最初のスタックアドレスから変更されていることが
    println!("Heap address of tuple data is: {:p}", b1);  // ヒープに対比したデータの先頭アドレスを参照
}