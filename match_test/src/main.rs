#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
}

#[derive(Debug)]
enum MyStudent {
    Wpx(Student),
    Wrj(Student),
    Zhangsan(Student),
    Lisi(Student),
}

fn match_student(s: &MyStudent) {
    match s {
        MyStudent::Wpx(s1) => {
            println!("age = {}", s1.age);
        },
        MyStudent::Wrj(s1) => {
            println!("name = {}", s1.name);
        },
        _ => (),  // 如果前两个都没有匹配到，那么，进到这个分支中，并且什么也不操作，直接返回 ()
    }
}

fn main() {
    let a = MyStudent::Wpx(
        Student {
            name: String::from("Wpx"),
            age: 24,
        }
    );
    println!("{a:?}");
    println!("{a:#?}"); // 更好看一点
    match_student(&a);
}
