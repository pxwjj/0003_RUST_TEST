fn test_vec_init() {
    let vtest = vec![1,2];
    let mut v1: Vec<f64> = Vec::new();
    v1.push(1.1);

    println!("{}", &v1[0]); // 使用引用，不使用引用也可以打印出值
    println!("{}", &v1[100]); // 导致程序painc

    match v1.get(0) {
        None => {
            println!("index error");
        },
        Some(num) => {
            println!("index success {num}");
        }
    }
}

fn test_vec_push(){
    let mut v1 = vec![1,2,3,4];
    let b_v1 = &v1[0]; // 如果是引用，那么 后续不能改变v1的值，
                             // 如果要添加元素长度不够需要更换一段内存空间，那么b_v1就会失效

    // v1.push(5); // 不能改变v1

    println!("{b_v1}");
}

fn test_vec_get(){
    let mut v1 = vec![1,2,3,4];
    
    for i in 0..v1.len() {
        println!("i = [{}]", v1[i]);

        match v1.get(i) {
            Some(num) => {
                println!("i = [{}]", num);
            },
            None => {}
        }
    }

    // 竟然可以这样做
    for v in v1 {
        println!("{v}");
    }

}

// 容器套枚举
fn test_vec_enum(){
    /*
        struct MyAddress_1 {
            Ip: String,
            Port: i32,
        }
    */
    #[derive(Debug)]
    enum MyAddress {
        Ip(String),
        Port(i32),
    }

    let wpx_addr = vec![
        MyAddress::Ip(String::from("127.0.0.1")),
        MyAddress::Port(9999)
    ];

    for i in wpx_addr {
        println!("{:#?}", i);
    }

}
fn main() {
    test_vec_enum();
}
