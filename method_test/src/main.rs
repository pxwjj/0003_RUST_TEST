use rand::Rng;

// 每次看一下自己的账户，钱都会增加
#[derive(Debug)]
struct bank_user {
    name: String,
    age: u32,
    money: u32,
}

impl bank_user {  // impl块中的所有内容都与 此结构体关联
    fn money_add (s: &mut Self) -> u32 {
        s.money += 1;
        return s.money;
    }
    
    fn get_money(&self) { // 注意 两个函数的入参，影响着，调用方式
        println!("{}", self.money);
    }

    fn can_hold (&self , b: &bank_user) -> bool { // 如果传入实例的money大于我 那么我就赢了（返回true）
        if self.money > b.money {
            return true;
        }
        false
    }

    fn init_money(m: u32) -> Self { // 关联函数。之前使用过String::from("") 初始化一个字符串
        bank_user {
            name: String::from("uesr_a"),
            age: 1, 
            money: m,
        }
    }
}

fn main(){
    let mut wpx = bank_user {
        name: String::from("wpx"),
        age: 24, 
        money: rand::thread_rng().gen_range(50000..=100000),
    };
    
    println!("money = {}", bank_user::money_add(&mut wpx));
    wpx.get_money();

    let wrj = bank_user {
        name: String::from("wpx"),
        age: 24, 
        money: rand::thread_rng().gen_range(50000..=100000),
    };

    match wpx.can_hold(&wrj) {
        true => {
            println!("me win");
        },
        flase => {
            println!("you win");
        },
    };

    let test: bank_user = bank_user::init_money(10000);
    println!("{:#?}", test);
}