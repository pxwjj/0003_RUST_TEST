#![allow(dead_code)]


// 修复错误，不要新增代码行
fn main1() {
    let _s: &str = "hello, world";
}

// 使用至少两种方法来修复错误
fn main2() {
    let s: Box<str> = "hello, world".into();    // "hello".into() 其实就是 Box::<str>::from("hello, world") 的语法糖！
                                                // 不光可以转换为String，
    greetings(&s)
}

fn greetings(s: &Box<str>) {
    println!("{}",s)
}

// 填空
fn main3() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");
}

// 修复所有错误，并且不要新增代码行
fn main4() {
    let mut s: String = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s)
}

// 填空
fn main5() {
    let s: String = String::from("I like dogs");
    // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
    let s1: String = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats")
}


// 修复所有错误，不要删除任何一行代码
fn main6() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // 加的时候， String + &str + &str ，第一个String 所有权进入add函数，退出add接口后，drop
    assert_eq!(s3,"hello,world!");
    // println!("{}",s1); // 加的时候 所有权被移走并drop
}