#![allow(dead_code)]

fn main1() {
    let x: i32 = 5;
    // 填写空白处
    let p: &i32 = &x;
 
    println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
 }

 
fn main2() {
    let x:i32 = 5;
    let y: &i32 = &x;

    // 只能修改以下行
    assert_eq!(5, *y);
}


// 修复错误
fn main3() {
    let _s = String::from("hello, ");

    // borrow_object(&s)
}

// fn borrow_object(_s: &String) {}


// 修复错误
fn main4() {
    let mut s: String = String::from("hello, ");
    let s: &mut String = &mut s;
    // push_str(s);
    println!("{}", s)
}

// fn push_str(s: &mut String) {
//     s.push_str("world")
// }

fn main5() {
    let mut s = String::from("hello, ");

    // 填写空白处，让代码工作
    let p: &mut String = &mut s;
    
    p.push_str("world");
}


fn main6() {
    let c: char = '中';

    let r1: &char = &c;
    // 填写空白处，但是不要修改其它行的代码
    let ref r2 = c; // ref也是引用的一种写法

    assert_eq!(*r1, *r2);
    
    // 判断两个内存地址的字符串是否相等
    assert_eq!(get_addr(r1),get_addr(r2));
}

// 获取传入引用的内存地址的字符串形式
fn get_addr(r: &char) -> String {
    let s: String = format!("{:p}", r); // format 有返回值，返回一个字符串
    s
}


// 移除代码某个部分，让它工作
// 你不能移除整行的代码！
fn main7() {
    let s = String::from("hello");

    let r1: &String = &s;
    let r2: &String = &s;

    println!("{}, {}", r1, r2);
}


fn main8() {
    // 通过修改下面一行代码来修复错误
    let mut s = String::from("hello, ");

    borrow_object(&mut s)
}

fn borrow_object(_s: &mut String) {}



// 注释掉一行代码让它工作
fn main9() {
    let mut s = String::from("hello, ");

    let r1: &mut String = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    
    // println!("{}",r1);
}

fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // 在下面增加一行代码人为制造编译错误：cannot borrow `s` as mutable more than once at a time
    // 你不能同时使用 r1 和 r2

    println!("{}", r1);
}