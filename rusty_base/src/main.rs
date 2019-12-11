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

//打印 i32类型value
fn print_value(value: i32) {
    println!("The value given was: {}", value);
}

//打印多个 i32类型value
fn print_values(value_1: i32, value_2: i32) {
    println!("Values given were: {} and {}", value_1, value_2);
}

//对参数进行加1运算，并打印运算后的结果，原参数值不变
fn increase_by_one(value: i32) {
    let mut value2 = value;
    value2 += 1;
    print_value(value2)
}

//分别对两个传感进行整除以及除余， 并返回两个结果值
fn divmod(x: i32, y: i32) -> (i32, i32) {
    return (x / y, x % y);
}
