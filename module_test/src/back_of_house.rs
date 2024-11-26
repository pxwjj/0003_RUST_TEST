pub struct Breakfast{
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: String) -> Breakfast {
        Breakfast {
            toast: toast,
            seasonal_fruit: String::from("apple"), // 在模块中可以设置 当前模块私有字段的值
        }
    }
}