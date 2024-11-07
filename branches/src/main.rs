use std::io::{self, Read};

fn main() {
    let a = 4;
    // if
    if a > 3 {
        println!("3 > 4");
    }else{
        println!("3 < 4");
    }

    if a == 1 {
        println!("a 为 1 ");
    } else if a == 2 {
        println!("a 为 2 ");
    } else if a == 3 {
        println!("a 为 3");
    } else {
        println!("a 为 4");
    }

    // 融合let 
    let bool_num: bool = true;
    let num = if bool_num {
                        4
                    }else {
                        5
                    };
    println!("num = {num}");

    // loop
    let mut count: u32 = 0;
    'counting_up: loop {
        println!("count = {count}");
        
        let mut count_1: u32 = 0;
        loop {
            println!("count_1 = {count_1}");
            count_1 += 1;

            if count_1 == 3 {
                break;
            }else if count == 2 {
                break 'counting_up;
            }
        }
        count += 1;
    }

    // while 
    let mut while_num: u32 = 10;
    while while_num > 0 {
        println!("while_num = {while_num}");

        while_num -= 1;
    }

    // for 
    let array_for: [i32; 5] = [1, 2, 3, 4, 5];
    for ele in array_for {
        println!("ele = {ele}");
    }

    for ele in (1..=4).rev() {
        println!("ele = {ele}");
    }

    println!("1 摄氏度 等于 {} 华氏度", convent_wendu(2.0));
    
    // 查看n个斐波那契数列
    loop {
        let mut f_num: String = String::new();

        println!("想查看第几个斐波那契数列呢，请输入");
        io::stdin()
            .read_line(&mut f_num)
            .expect("reinput f_num");

        let mut f_num: u32 = match f_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("string to num error,reinput");
                continue;
            },
        };
        println!("第{f_num}个斐波那契数为： {}", feibonaqie(f_num));
    }
}

// 摄氏度转为华氏度 
fn convent_wendu (s: f64) -> f64 {
    s * 1.8 + 32.0
}

// 从第三项开始，每一项等于前两项之和
fn feibonaqie (i: u32) -> u32 {
    if i < 0 {
        return 0;
    } else if i < 3 {
        return 1;
    }else {
        return feibonaqie(i-1) + feibonaqie(i-2);
    }
}