// struct v4_struct {
//     x: String,
//     y: u32,
// }

// struct v6_struct {
//     i: String,
//     j: u32,
// }

//---
enum ip_addr_kind {
    v4,
    v6,
}

struct ip_addr {
    kind: ip_addr_kind, // 结构体融合了enum
    value: String,
}

fn test(){ // 创建 结构体嵌套enum 的实例
    let four: ip_addr = ip_addr {
        kind: ip_addr_kind::v4, // enum的实例 注意 ::
        value: String::from("127.0.0.1"),
    };

    let six: ip_addr = ip_addr {
        kind: ip_addr_kind::v6,
        value: String::from("::1"),
    };
}

//---
enum ip_addr_kind1{ // 将数据附加到每一个枚举成员上
    v4(String),
    v6(String),
}

fn test1(){
    let four: ip_addr_kind1 = ip_addr_kind1::v4(String::from("127.0.0.1"));
    let six: ip_addr_kind1 = ip_addr_kind1::v6(String::from("::1"));
}

//---

enum message {
    quit,
    my_move{ x: i32, y: i32 },
    write(String),
    changecolor(i32, i32, i32),
}

impl message {
    fn call(&self) {
        // 函数体
    }
}

fn test2(){
    let a: message = message::quit;
    let b: message = message::my_move { x: 1, y: 1 };
    let c: message = message::write(String::from("value"));
    let d: message = message::changecolor(1, 1, 1);
    
}


fn main(){
    println!("a");
    let my_message:message = message::changecolor(1, 1, 1);
    my_message.call();

    let a = Option::Some(1);
    let b: Option<i32> = Option::None;
}