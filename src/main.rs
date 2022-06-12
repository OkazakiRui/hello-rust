mod vars;

fn main() {
    println!("Hello, world!");
    vars::run();
    vars::a::a();
    vars::b::a();
}
