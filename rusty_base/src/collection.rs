

//打印 普通数组 
pub fn studyArr(){
    //直接定义数组
    // let arr = [1,2,3];
    let arr = ["1","2","3"];
    println!("{:?}",arr);
    //访问数组第一个元素
    println!("{}",arr[0]);

    //初始化数组大小  元素大小必须与元素个数一致
    let arr1:[i32;5] = [1,2,3,4,5];
    println!("{:?}",arr1);//[1,2,3,4,5]

    //数组定义为不可变，如果这里执行，则报错
    // arr1[0]=98

    //定义可变数组
    let mut arr2:[i32;3] = [3,4,5];
    arr2[0]=1;
    println!("{:?}",arr2); //1,4,5

    // 定义初始化   定义元素成员都是一样的值且值是字符串100,也可为数字
     let all100 = ["100";5];
     println!("{:?}",all100);

     //遍历数组
     for item in arr1.iter() {
         println!("{}",item)
     }


     //数组切片
     let arr5 = [100,4,5,6,6];
     let arr4 = &arr5[0..3];  //截取0 1 2 元素成员，也就是 100 4 5
     println!("{:?}",arr5); //原来的数组不变 [100, 4, 5, 6, 6]
     println!("{:?}",arr4); // [100, 4, 5]

     //数组长度
     println!("{:?}",arr4.len());
}

//动态数组Vec
pub fn studyVector(){
    //new关键字实例化一个动态数组，push方式添加成员
    let mut vector1 = Vec::new();
    vector1.push(5);
    println!("{:?}",vector1);

    //
    let mut vector2 = vec![10,13,11];
    let num = vector2[1];
    println!("{:?}",vector2);
    println!("{}",num);

    //相当于 switch if 语句，匹配成员
    match vector2.get(1) {
        Some(third) => println!("The one element is {}", third),
        None => println!("There is no one element."),
    }

    // 定长vec遍历   定长是因为没有通过mut关键字修饰，是不可变类型
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    //可变Vec遍历
    let mut changeV2 = vec![100, 32, 57];
    //因为遍历过程中需要改变成员值，所以需要 &mut修饰(&mut changeV2)
    //也可以let一个新的变量，通过成员计算 赋值给新变量
    for i in  changeV2 {
        // *i += 50;
        let itmp = i+50;
        println!("{}",itmp);
    }

}


//TODO 不可重复数组 类似Set 双链表