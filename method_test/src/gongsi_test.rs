/*
题目 1：定义一个Rectangle结构体
    描述：定义一个Rectangle结构体，包含width（宽度）和height（高度）两个字段。
    然后，定义一个方法area()，返回矩形的面积。

要求：
    定义Rectangle结构体。
    使用impl块为Rectangle结构体实现方法area()，该方法返回矩形的面积（width * height）。
    在main函数中创建一个Rectangle实例，并调用area()方法打印面积。
*/

/* 包含的知识点
    1、结构体： 声明以及创建实例
    2、方法：用.调用的，用::调用的，注意声明时候的区别
    3、loop：嵌套，loop标签的使用
    4、stdin：控制台输入
    5、match使用
    6、隐藏变量用法
    7、parse：字符串解析为数字
*/
use std::{io::stdin, thread::sleep, time::Duration};

struct Rectangle {
    length: f64, 
    width: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.length * self.width
    }

    fn init_rectangle(l: f64, w: f64) -> Rectangle { // Rectangle在impl中跟Self一样的意思
        let r: Rectangle = Rectangle {
            length: l,
            width: w,
        };
        return  r;
    }
}
fn main() {
    'first_loop: loop { // 注意加冒号
        // 1. input l
        println!("input l");
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("input err");
        let l: f64 = match buf.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("reinput l");
                continue 'first_loop;
            },
        }; 

        'sec_loop: loop {
             // 2. input w
            println!("input w");
            buf.clear();
            stdin().read_line(&mut buf).expect("input err");
            let w: f64 = match buf.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("reinput w");
                    continue 'sec_loop; //continue 也能使用loop标签 ,也可以在w错误之后，回到第一层loop： continue 'first_loop
                },
            };
            // 3. init Rectangle
            let rect = Rectangle::init_rectangle(l, w);

            // 4. print rect area
            println!("{}", rect.area());
            break 'sec_loop; //算完之后，记得跳出第二层loop
        }
        sleep(Duration::from_secs(1));
    }
}
