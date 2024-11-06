use rand::Rng;
use std::{io,cmp::Ordering};

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
}