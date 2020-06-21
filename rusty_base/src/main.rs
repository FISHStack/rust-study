pub mod collection;
pub mod map;
pub mod set;
pub mod typeconv;

fn main() {
    let (x, y) = (5, '6');
    println!("Hello, world! x:{},y:{}", x, y);
    let mut pv: i32 = 66;
    pv = pv + 1;
    print_value(pv);
    increase_by_one(pv);
    // print_value(pv);
    let pv2: i32 = pv + 1;
    print_value(pv2);

    let (x1, y1) = divmod(pv, pv);
    print_values(x1, y1);

    expAndPrint(10);

    let mut s = String::from("hello world");

    print_context(&s);

    // str2String("hello")

    //使用tyeconv文件里面的str2String函数
    print_context_String(typeconv::str2String(&s));

    typeconv::String2Str(&*s);

    let hour: i32 = 12;

    //i32整形转字符串
    print_context_String(typeconv::int2String(hour));

    let strTmp = String::from("14");

    print_value(typeconv::String2int(strTmp));

    collection::studyArr();

    collection::studyVector();

    map::studyHashMap();

    collection::studyVecDeque();

    collection::studyLinkedList();

    map::sutdyBTreeMap();

    set::studyHashSet();
}

//打印 i32类型value
fn print_value(value: i32) {
    println!("The value given was: {}", value);
}

//打印 str类型value
fn print_context(context: &str) {
    println!("str context: {}", context);
}

//打印 string类型value
fn print_context_String(context: String) {
    println!("string context: {}", context);
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

fn expAndPrint(sum: i32) {
    if sum > 5 {
        print_context("sum > 5")
    } else {
        print_context("sum <=5")
    }
}

//  typeconv.rs context
