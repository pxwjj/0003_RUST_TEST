use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // 生成随机数
    let rand_num: i32 = rand::thread_rng().gen_range(1..=100);
    
    loop {
        // 1、输入字符串
        println!("input:");
        let mut in_buf: String = String::new();
        io::stdin()
                .read_line(&mut in_buf)
                .expect("err");
    
        // 2、将字符串处理，变为数字
        //  2.1 如果转换失败，重新输入
        let in_buf:i32 = match in_buf.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        // 3、比较大小，相等则成功
        match rand_num.cmp(&in_buf) {
            Ordering::Less => println!("再小点"),
            Ordering::Greater => println!("再猜大点"),
            Ordering::Equal => {
                println!("you win!");
                break;
            },
        }
    }
}

