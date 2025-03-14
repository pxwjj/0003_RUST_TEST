#![allow(dead_code)]

// 修改2处 `assert_eq!` 让代码工作

use std::mem::size_of_val;
fn main1() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4); 

    println!("Success!")
} 

// 修改一行让代码正常打印
fn print_char(c : char) {
    println!("{}", c);
}

fn main2() {
    let c1 = '中';
    print_char(c1);
} 


// 使成功打印
fn main3() {
    let _f: bool = false;

    let t = true;
    if t {
        println!("Success!")
    }
} 

fn main4() {
    let f = true;
    let t = true || false;
    assert_eq!(t, f);

    println!("Success!")
}

fn implicitly_ret_unit() {
    println!("I will return a ()")
}

// 不要使用下面的函数，它只用于演示！
fn explicitly_ret_unit() -> () {
    println!("I will return a ()")
}

// 让代码工作，但不要修改 `implicitly_ret_unit` !
fn main5() {
    let v = (2, 3);
    println!("{:?}", v);

    let v: () = ();
    assert_eq!(v, implicitly_ret_unit());

    println!("Success!")
}


// 让代码工作：修改 `assert!` 中的 `4` 
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!")
}