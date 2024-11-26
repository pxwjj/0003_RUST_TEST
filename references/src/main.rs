
fn alg_string_len(str: &String) ->usize {
    str.len()
}

fn change_str(str: &mut String){ // 入参为可变引用
    str.push_str(" string");
}

fn main() {
    let s: String = String::from("value");
    let len: usize = alg_string_len(&s); // 传入s的引用，不给所有权。因此在函数调用之后还能使用s
    println!("srting = {s} len = {len}");

    let mut s1: String = String::from("value"); // 可变字符串
    change_str(&mut s1); // 可变引用
    println!("change after {s1}");

    {
        let mut s2: String = String::from("value");

        let r_s1: &mut String = &mut s2;
        let r_s3: &mut String = &mut s2; // 不能两个可变引用同时作用于同一个可变字符串上
    }

    // {
    //     // 悬垂引用例子
    //     let refrence_str = ref_dangle();
    // }
}

// fn  ref_dangle() -> &String {
//     let s: String = String::from("value");
//     &s
// } // s借用给了，refrence_str。 ，但是出了作用域已经被drop，此时refrence_str是一个悬垂引用