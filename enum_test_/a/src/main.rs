/*
定义一个 struct 类型 Person，
它包含一个可选的字段 nickname（Option<String>）。
编写一个函数 greet，它会打印出 Person 的问候语。
如果 nickname 存在，打印出昵称；
如果 nickname 不存在，则打印出默认的问候语。


要求：
使用 Option<String> 来表示 nickname 字段。
使用 match 来匹配 Option 类型并根据 Some 或 None 分支输出不同的消息。
*/

struct MyPerson {
    name: String,
    nickname: Option<String>,
}

impl MyPerson {
    fn say(&self){
        match &self.nickname {
            None => {
                println!("hello, how are you?");
            },
            Some(s) => {
                println!("{}", s);
            },
        }
    }
}

fn main() {
    let p = MyPerson {
        name: String::from("wang peng xiang"),
        nickname: Some(String::from("xie&bro")), // Option 初始化
    };

    let p1 = MyPerson {
        name: String::from("wang rong jing"),
        nickname: None,
    };
    
    p.say();
    p1.say();
}
