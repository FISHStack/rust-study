
use std::collections::HashMap;

use std::collections::BTreeMap;


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

pub fn sutdyBTreeMap(){
    //https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
    let mut btreeMap = BTreeMap::new();
    // review some movies.
    btreeMap.insert("Office Space",       "Deals with real issues in the workplace.");
    btreeMap.insert("Pulp Fiction",       "Masterpiece.");
    btreeMap.insert("The Godfather",      "Very enjoyable.");
    btreeMap.insert("The Blues Brothers", "Eye lyked it a lot.");

    

    // check for a specific one.
    if !btreeMap.contains_key("Les Misérables") {
        println!("We've got {} reviews, but Les Misérables ain't one.",
        btreeMap.len());
    }


    // look up the values associated with some keys.
    let to_find = ["Up!", "Office Space"];
    for movie in &to_find {
        match btreeMap.get(movie) {
        Some(review) => println!("{}: {}", movie, review),
        None => println!("{} is unreviewed.", movie)
        }
    }

    fn random_stat_buff() -> String {
        // &*"42".to_string()
        let s = String::from("hello");
        s
    }

    // fn random_stat_buff() -> i32 {
    //     // could actually return some random value here - let's just return
    //     // some fixed value for now
    //     42
    // }

    // insert a key only if it doesn't already exist
    btreeMap.entry("health").or_insert("100");

    // insert a key using a function that provides a new value only if it
    // doesn't already exist

    let ranStr = random_stat_buff;

    // btreeMap.entry("defence").or_insert_with(2);

    for item in btreeMap{
        println!("for.map:{}|{}",item.0,item.1);
    }

    let mut btreeNumMap = BTreeMap::new();
    for i in 11..21{
        btreeNumMap.insert(i,i);
    }

    for i in 1..10{
        btreeNumMap.insert(i,i);
    }

    //youxu
    for item in btreeNumMap{
        println!("for.nummap:{}|{}",item.0,item.1);
    }
    

}