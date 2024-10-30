use std::io;
use rand::Rng;

fn main() {
    let mut buf = String::new();
    
    let rand_num = rand::thread_rng().gen_range(1..=100);
    println!("{rand_num}");

    println!("guess process!!");
    println!("input:");

    io::stdin()// return fd
        .read_line(&mut buf)// fd 的 method
        .expect("error"); // read_line 的返回值 result 的 method
}