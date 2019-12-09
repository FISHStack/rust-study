fn main() {
    let (x,y) = (5,'6');
    println!("Hello, world! x:{},y:{}",x,y);
    let mut pv :i32 = 66 ;
    pv = pv +1;
    print_value(pv);
    increase_by_one(pv);
    // print_value(pv);
    let pv2:i32 = pv+1;
    print_value(pv2);

    let (x1,y1) = divmod(pv,pv);
    print_values(x1,y1);
}

fn print_value(value: i32) {
    println!("The value given was: {}", value);
}

fn print_values(value_1: i32, value_2: i32) {
    println!("Values given were: {} and {}", value_1, value_2);
}

fn increase_by_one(value: i32) {
    let mut value2 = value;
    value2 += 1;
    print_value(value2)
}

fn divmod(x: i32, y: i32) -> (i32, i32) {
    return (x / y, x % y);
}
