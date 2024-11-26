fn main() {
    {
        let s: String = String::from("hello"); // 从此处起 s有效
        
        // 使用s 
    }   
    // s不在有效 默认调用了 drop 函数

    {
        // String 由 指针，长度，容量组成
        // 指针： 指向堆
        // 长度为： 使用长度
        // 容量为： 总共获取了多少字节
        let s1: String = String::from("value");
        
        /* 
        * 该语句执行完后，s1不在生效，自动被释放
        * s2指向s1分配的内存。
        * 该行为不能称为拷贝，称为移动 move。
        */
        let s2 = s1; 
    } //离开作用域后，只drop s2
    
    {
        let s1: String = String::from("value");
        let s2 = String::clone(&s1); // clone 深拷贝

        println!("{s1} {s2}");
    } // 会drop s1 和 s2

    {
        // 所有权与函数
        let s1: String = String::from("value");
        test_ownership(s1); // s1的所有权给something , 调用完此函数之后，不能再使用s1
        // println!("s1 = {s1}"); // 报错

        let a: u32 = 1;
        test_u32_cope(a); // 因为u32为cope类型，所以，a还能使用。并不是把a的所有权交给num。
        println!("a = {a}");
    }
}

fn test_ownership(something: String){
    println!("something = {something}");
} // 调用drop somtthing 占用的内存被释放

fn test_u32_cope(num: u32){
    println!("num = {num}");
}