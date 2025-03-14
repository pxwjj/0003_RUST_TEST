#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_must_use)]

fn main1() {
    let x = 5u32;

    let y = {
        let x_squared = x * x; // 25
        let x_cube = x_squared * x; //125

        // 下面表达式的值将被赋给 `y`
        x_cube + x_squared + x //125 + 25 + 5
    };

    let z = {
        // 分号让表达式变成了语句，因此返回的不再是表达式 `2 * x` 的值，而是语句的值 `()`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

// 使用两种方法让代码工作起来
fn main2() {
    let v = {
        let mut x = 1;
        x + 2
    };
 
    assert_eq!(v, 3);
 }
 
fn main3() {
    let v = {
        let _x = 3;
    };
 
    assert!(v == ());
 }

 
fn main() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}