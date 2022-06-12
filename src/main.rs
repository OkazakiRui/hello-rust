mod vars;

// staticに格納される
const MAX_POINTS: u32 = 100_000;

fn main() {
    // println!("Hello, world!");
    vars::run();
    // vars::a::a();
    // vars::b::a();

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
}
