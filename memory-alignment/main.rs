use std::mem;
struct A {
    a: u8,
    b: u16,
    c: u8,
}

fn main() {
    let aa = A { a: 1, b: 2, c: 3 };
    println!("size = {}", mem::size_of_val(&aa));
    println!("size = {}", mem::size_of::<A>());
    println!("Hello, world!");
}
