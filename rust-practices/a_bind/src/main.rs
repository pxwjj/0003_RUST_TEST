#![allow(dead_code)]

// 修复下面代码的错误并尽可能少的修改
fn main1() {
    let x: i32 = 1; // 未初始化，但被使用
    let _y: i32; // 未初始化，也未被使用
    println!("x is equal to {}", x); 
}

// 完形填空，让代码编译
fn main2() {
    let mut x = 1;
    x += 2; 
    
    println!("x = {}", x); 
}

// 修复下面代码的错误并使用尽可能少的改变
fn main3() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("x 的值是 {}, y 的值是 {}", x, y);
    }
    println!("x 的值是 {}", x); 
}

// 修复错误
fn main4() {
    define_x();
}

fn define_x() {
    let x = "hello";
    println!("{}, world", x); 
}


// 只允许修改 `assert_eq!` 来让 `println!` 工作(在终端输出 `42`)
fn main5() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // 输出 "42".
}


fn main6() {
    let _x = 1; 
}
// compiler warning: unused variable: `x`


// 修复下面代码的错误并尽可能少的修改
fn main7() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
}


fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // 填空，让代码工作
    assert_eq!([x,y], [3,2]);
} 