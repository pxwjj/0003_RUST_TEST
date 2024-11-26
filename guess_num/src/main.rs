use rand::Rng;
use std::{io,cmp::Ordering};

fn print_hour (num: u32, ocloc: char) {
    println!("how time? {num}{ocloc}");
}

fn num1_plus_num2 (num1: u32, num2: u32) -> u32 {
    num1 + num2  // don`t use ';' 否则 就是语句了，不是表达式了。
}

fn main (){
    print!("please input a number between 0 to 100:\n");

    // 1. rand num
    let r_num: u32 = rand::thread_rng().gen_range(0..101);

    // 2. loop 
    loop {
        // 2.1 write num
        let mut w_num: String = String::new();
        io::stdin()
            .read_line(&mut w_num)
            .expect("err");

        // 2.2 string to num , err or ok
        let w_num: u32 = match w_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("input isn`t num string,reinput");
                continue;
            },
        };

        // 3.cmp
        match w_num.cmp(&r_num) {
            Ordering::Equal => {
                println!("you win!!!");
                break;
            },
            Ordering::Greater => {
                println!("reinput small:");
                continue;
            },
            Ordering::Less => {
                println!("reinput big:");
                continue;
            },
        };
    }

    // 元组 - tuple
    let t: (i32, f64, bool) = (2, 0.1, true);
    let (x, y, z) = t;
    println!("y = {y}"); // 0.1
    println!("t.0 = {}, t.1 = {}, t.2 = {}", t.0, t.1, t.2); // 2, 0.1, true

    // 数组 - array
    let a: [i32; 4] = [1, 2, 3, 4];
    println!("a[0] = {}", a[0]); // 1

    print_hour(2, 'h');
    println!("1+1 = {}", num1_plus_num2(1, 1));

    // 表达式
    let wpx = {
        let wrj: u32 = 1;
        wrj + 1 // 如果加了‘；’那么返回值就是 () 了。语句的返回值，默认就是这个玩意儿
    };

    println!("wpx = {wpx}");
}