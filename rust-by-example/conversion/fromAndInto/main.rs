use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {

    let my_str = "Hello world";
    let my_string = String::from(my_str);


    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    // Try removing the type declaration
    let num2: Number = int.into();
    println!("My Into number is {:?}", num2);
}

