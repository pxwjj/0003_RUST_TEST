// 该函数接收一个用空格分隔单词的字符串， 返回该字符串中的第一个单词。
fn first_word_index(s: &String) -> usize {
    let bytes_array = s.as_bytes();
    for (i, &item) in bytes_array.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}

fn first_word(s: &String) -> &str { // 传引用，返回切片（一个字符串中的一部分）
    let bytes_array = s.as_bytes(); // 当作字节数组，然后再操作
    for (i,&item) in bytes_array.iter().enumerate() { // 用数组迭代器，并且返回index 和 指针。迭代器返回的是引用
        if item == b' ' { // b' ' 表示空格的字节值
            return &s[..i]; // 第i位为空格， &s[0..10] 不包含index 为10的位置
        }
    }
    return &s[..]; 
}

// 更加通用，入参可以传切片
fn first_word_good(s: &str) -> &str {
    let bytes_array = s.as_bytes();
    for (i, &item) in bytes_array.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

fn main() {
    let s_test: String = String::from("pxwjj wrj");
    let first_string = first_word(&s_test);

    println!("{first_string}");

    let slice_test1: &str = &s_test[..];
    let a = "hello";
}
