pub fn run() {
    // let a1: [u8; 9000000] = [1; 9000000];
    // let a1: [u8; 7000000] = [1; 7000000];

    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];

    println!("v1 is pointer address : {:p}", &v1);
    println!("v2 is pointer address : {:p}", &v2);
    // 0x7ffeecdd8bf0 - 0x7ffeecdd8bd8 = 24bytes
    // String型と同じメモリ構造をしている

    println!("heap address : {:?}", v1.as_ptr());
    println!("len : {}", v1.len());
    println!("capacity : {}", v1.capacity());
    v1.insert(1, 10);
    println!("{:?}", v1);
    v1.remove(0);
    println!("{:?}", v1);

    v1.append(&mut v3);
    println!("{:?}", v1);
    println!("{:?}", v3);
}
