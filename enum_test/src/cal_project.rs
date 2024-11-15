/*
目标：
    实现一个基本的计算器，可以执行加法、减法、乘法、除法运算，并处理除以零的情况。这个项目会练习循环、match、引用和切片。

任务说明：
    创建一个Calculator结构体，包含add、subtract、multiply和divide等方法
    允许用户输入多个计算式子，使用循环来处理用户的输入。
    处理除法中除以零的情况。
*/
use std::{io::stdin};

struct MyQuation {
    n1: f64,
    n2: f64,
    s: String,
}

enum MyCal {
    MyAdd(MyQuation),
    MySubtract(MyQuation),
    MyMultiply(MyQuation),
    MyDivide(MyQuation),
}

impl MyCal { // 每一种变体对应一种方法
    fn cal_quation(&self) -> Option<f64> {
        match self {
            MyCal::MyAdd(q) => {  // 注意不是这样写的： self::MyAdd(q) 在这里耗费差不多1-2小时
                print!("{} + {} = ", q.n1, q.n2);
                Some(q.n1 + q.n2)
            },
            MyCal::MySubtract(q) => { 
                print!("{} - {} = ", q.n1, q.n2);
                Some(q.n1 - q.n2)
            },
            MyCal::MyMultiply(q) => { 
                print!("{} * {} = ", q.n1, q.n2);
                Some(q.n1 * q.n2)
            },
            MyCal::MyDivide(q) => { 
                if q.n2 == 0.0 {
                    return None;
                }
                print!("{} / {} = ", q.n1, q.n2);
                Some(q.n1 * q.n2)
            },
        }
    }
}

fn my_read_line_num() -> f64 {
    loop {
        let mut num = String::new();
        stdin().read_line(&mut num).expect("err: read num");
        let num: f64 = match num.trim().parse() {
            Ok(num) => {
                break num;
            },
            Err(_) => {
                println!("请输入数字！！！");
                continue;
            },
        };
    }
}

fn my_read_line_symbol(n1: f64, n2: f64) -> MyCal {
    loop {
        let mut symbol: String = String::new();
        stdin().read_line(&mut symbol).expect("err: read symbol");
        
        match symbol.trim() { //注意去掉空格
            "+" => { 
                let my_cal = MyCal::MyAdd( MyQuation { n1: n1, n2: n2, s: symbol, } );
                return my_cal;
            },
            "-" => { 
                let my_cal = MyCal::MySubtract( MyQuation { n1: n1, n2: n2, s: symbol, } );
                return my_cal;
            },
            "*" => { 
                let my_cal = MyCal::MyMultiply( MyQuation { n1: n1, n2: n2, s: symbol, } );
                return my_cal;
            },
            "/" => { 
                let my_cal = MyCal::MyDivide( MyQuation { n1: n1, n2: n2, s: symbol, } );
                return my_cal;
            },
            _ => {
                println!("请输入正确的运算符！！！");
                continue;
            },
        }
    }
}

fn main() {
    loop {
        println!("input num1");
        let num1 = my_read_line_num();
        println!("input num2");
        let num2 = my_read_line_num();
        println!("input + - * /");

        let my_cal = my_read_line_symbol(num1, num2); // Option<> 的使用
        match my_cal.cal_quation() {
            None => {
                println!("运算式错误，比如被除数为0，请重试！！！");
            },
            Some(result) => {
                println!("{result}");
            },
        }

        println!("再来一次请输入 ： y");
        let mut again = String::new();
        stdin().read_line(&mut again).expect("read line err");
        if again.trim().to_lowercase() != "y".to_string() {
            break;
        }
    }
}
