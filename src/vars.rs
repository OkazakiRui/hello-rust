pub mod a;
pub mod b;

pub fn run() {
    // println!("here is vars module");
    a::a();
    b::a();

    let mut a: i8 = 10;
    let b = 20;
    println!("{}", a + b);
    a = 6;
    println!("{}", a + b);

    let _hoge = "hoge";

    println!("{}", usize::BITS);

    println!("{:p}", &MAX_POINTS);

    // stackに格納される
    let i2: i64 = 1; // i64は8bytes分のメモリを確保する
    let i3: i64 = 2;
    println!("{:p}", &i2);
    println!("{:p}", &i3);
    // 0x7ffee758dc28
    // 0x7ffee758dc30

    // バインドを複数回行った場合は、stack内にて別のアドレスが確保される
    let y = 5;
    println!("{:p}", &y);
    let y = y + 1;
    println!("{:p}", &y);
    let y = y * 2;
    println!("{:p}", &y);
    println!("{}", y);
    {
        let y = 0;
        println!("{}", y);
    }
    println!("{}", y);

    // タプル型
    let t1 = (500, 4.6, "hoge");
    let (x, y, z) = t1;
    println!("{}, {}, {}", x, y, z);
    println!("{}, {}, {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));
    println!("{:?}", t2);
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    *x1_ptr = 10;
    *y1_ptr = -10;
    println!("{:?}", t2);

    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]);

    let s1 = "helloこんにちは挨拶"; // 26bytes
    let s2 = "hello"; // 5bytes

    // 16bytes分のメモリが確保されている。
    // s1は26bytesなのになぜ？
    // 1~8: アドレス
    // 9~16: 長さ
    println!("{:p}", &s1); // 0x7ffee2813cc8
    println!("{:p}", &s2); // 0x7ffee2813cd8

    // 文字列スライス型の前8bytesに格納されている、メモリ番地を取得する
    println!("{:?}", s1.as_ptr()); // 0x1028838b4
    println!("{:?}", s2.as_ptr()); // 0x1028838ce
                                   // 0x1028838b4 - 0x1028838ce = 0x1A
                                   // 0x1A = 16 + 10 = 26
                                   // 26bytes分ずれている
    println!("{:?}", s1.len()); // 26bytes
    println!("{:?}", s2.len()); // 5bytes

    // string型を作成していく
    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloworld");
    println!("{:p}", &s1); // 0x7ffeea9ceca8
    println!("{:p}", &s2); // 0x7ffeea9cecc0
                           // 24bytesのメモリが確保されている
                           // 1~8: アドレス
                           // 9~16: 長さ
                           // 17~24: キャパシティ

    // アドレス
    println!("{:?}", s1.as_ptr());
    println!("{:?}", s2.as_ptr());
    // 長さ
    println!("{}", s1.len());
    println!("{}", s2.len());
    // キャパシティ
    println!("{}", s1.capacity());
    println!("{}", s2.capacity());
    s1.push_str("hoge");
    s2.push_str("fugafugapiyopiyo");
    println!("{}", s1.capacity());
    println!("{}", s2.capacity());
    println!("{} {}", s1, s2);
}
