struct User {
    name: String,
    age: u32,
    email: String,
    active: bool,
}

#[derive(Debug)] // 不添加此宏，println!打印不出来
struct Rectangle {
    length: u32, 
    width: u32,
}

fn Rectangle_area(R: &Rectangle) -> u32 {
    R.length * R.width
}

fn creat_user(name1: String, age1: u32, email1: String, active1: bool) -> User {
    let user_test = User {
        name: name1,
        age: age1, 
        email: email1, 
        active: active1,
    };

    return user_test;
}

fn main() {
    let wpx: User = User {
        name: String::from("wpx"),
        age: 24,
        email: String::from("1272149266@qq.com"),
        active: true,
    };
    println!("wpx.name = {}", wpx.name);

    let mut wrj: User = User {
        name: String::from("wrj"),
        age: 26,
        email: String::from("wrj1017@google.com"),
        active: false,
    };
    wrj.active = true;
    print!("wrj.active = {}\n", wrj.active);

    let hfl = creat_user(
        String::from("hfl"), 32, 
        String::from("1010101@google.com"), true
    );
    println!("hfl.email = {}", hfl.email);

    let R: Rectangle = Rectangle {
        length: 2,
        width: 10,
    };
    println!("{}",Rectangle_area(&R));

    // 查看结构体实例的值
    println!("{:?}", R);
    println!("{:#?}", R);

}

