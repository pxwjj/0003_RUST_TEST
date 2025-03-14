#![allow(dead_code)]

fn main1() {
    // 不要修改下面两行代码!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
fn main2() {
    print();
 }
 
 // 使用另一个类型来替代 i32
 fn print() -> () {
    println!("hello,world");
 }

 // 发散函数 不返回任何值
 // 用两种方法求解
fn main3() {
    never_return();
}

fn never_return() -> ! {
    // 实现这个函数，不要修改函数签名!
    // loop {};
    panic!("I return nothing!");
}


// ----
fn main4() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // 这里与其返回一个 None，不如使用发散函数替代
    never_return_fn()
}

// 使用三种方法实现以下发散函数
fn never_return_fn() -> ! {
    // unimplemented!();
    // panic!()
    // todo!();
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1))
    }
}


fn main() {
    // 填空
    let b = false;

    let _v = match b {
        true => 1,
        // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic")
        }
    };

    println!("Exercise Failed if printing out this line!");
}