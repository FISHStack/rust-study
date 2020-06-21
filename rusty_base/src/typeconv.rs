//将函数暴露 公共范围使用 pub 关键字
//将&str类型转换成String类型
pub fn str2String(context: &str) -> String {
    return context.to_string();
}

//外部通过  &*String 方式将String类型转换成&str类型   也就是说 &*String 可以转换成&str类型
pub fn String2Str(strContext: &str) {
    println!("convert String to &str {}", strContext);
}

//i32类型的数字转换成String类型，u32 i64也是如此
pub fn int2String(number: i32) -> String {
    return number.to_string();
}

//String类型转i32数字
pub fn String2int(context: String) -> i32 {
    return context.parse::<i32>().unwrap();
}
