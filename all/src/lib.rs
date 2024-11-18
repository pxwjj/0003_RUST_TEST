fn have_name(){}


mod people {
    pub mod student {
        pub fn get_student_name(){}
        fn get_student_age(){}
        mod daxue {
            fn get_daxue_name(){}
        }
    }

    mod worker {
        fn get_worker_name(){}
        fn get_worker_age(){}
    }
}

/*
    crate
    |
    | - - - people
            |
            | - - - pub student
            |       |
            |       | - - - pub get_student_name
            |       | - - - get_student_age
            |       | - - - daxue
            |               | 
            |               | - - - get_daxue_name
            |
            | - - - worker
                    |
                    | - - - get_worker_name
                    | - - - get_worker_age
*/

pub fn get(){
    // 绝对路径
    crate::people::student::get_student_name();
    // 相对路径
    people::student::get_student_name();
}


/*
    back_of_house
        Breakfast
            toast string
            seasonal_fruit string

            fn summer -- in:toast out:Breakfast

*/
mod back_of_house{
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
}


fn eat_at_restaurant(){

    let mut meal = back_of_house::Breakfast::summer(String::from("Rye")); // 将结构体变为pub，对应的summer变为pub

    meal.toast = String::from("Wheat"); // 更改结构体中参数 将结构体成员变为 pub -- 结构体共有不是说结构体所有成员都共有

    // meal.seasonal_fruit =  String::from("banana");// 不能编译，因为seasonal_fruit 不是共有成员
}