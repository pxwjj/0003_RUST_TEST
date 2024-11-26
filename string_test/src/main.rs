fn test_string_init() {
    let s1 = String::from("value");
    let s2 = "value".to_string();
    // 以上两种 是同一种方式，看使用习惯
}

fn test_string_modify() {
    let mut s2 = "value".to_string();
    s2.push_str(" a"); // 追加string , 不会获取所有权
    println!("{s2}");

    s2.push('a'); // 追加 char
    println!("{s2}");
}

fn test_string_puls() {
    let s1 = "s1".to_string();
    let s2 = "s2".to_string();

    let s3 = s1 + &s2; // 只能这样写，第一个移动，第二个引用。 与add函数入参有关
    // s1不能使用了
    println!("{}", &s3[0..]);
    println!("{}", &s3); // &str[0..] == &String
}

// 使用format 代替plus
fn test_string_format() {
    let s1 = "s1".to_string();
    let s2 = "s2".to_string();

    let s3 = format!("{s1} 杂七杂八 {s2}");
    println!("{s3}");
}

fn test_string_index() {
    let hello = "こんにちは".to_string();
    // 不是每个字符都是占用一个字节，因此不能索引
    // hello[0]; //编译不通过

    // 但是可以使用 slice
    println!("{}", &hello[0..=2]); // 返回 こ ，可以发现，日文的一个字符占用三字节

    // -- 
    for i in hello.chars() {// 分别将 こ ん に ち は 拆分出来，打印
        println!("{i}");
    }
    for i in hello.bytes() { // 这个会打印出来每个字节 ,一共打印 3*5 个字节
        println!("{i}");
    }
}

fn main() {
    test_string_index();
}
