
use std::collections::HashMap;


//map相关的学习 hashmap treemap linkmap

pub fn studyHashMap(){
    //实例化一个hashmap
    let mut map = HashMap::new();
    //往hashmap插入元素
    map.insert("company","gm");
    map.insert("name","lan");
    map.insert("sex","女");
    //插入失败，只能插入字符串类型
    // map.insert("age",24);

    let select = "hello";

    if !map.contains_key(select){
        println!("map 没有 {} 这个成员",select);
    }

    //删除一个不存在的成员
    map.remove("world");
    //删除一个存在的成员
    map.remove("company");

    println!("{:?}",map.get("haha"));  //None

    //if this after code B, and exception err
    for item in map.iter(){
        println!("map.member:{},index:{}",item.0,item.1);
    }

    //for code B
    for (item,index) in map{
        //删除了company成员，剩下name和sex
        println!("map.member:{},index:{}",item,index);
        //在for遍历中，不能使用map.get()  所有权相关
    };

    //may delete member
    // map.retain(|k, v| {
        // *k % 2 != 0
    // });
    
}