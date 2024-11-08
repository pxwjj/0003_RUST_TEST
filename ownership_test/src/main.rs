
fn main() {
    println!("Hello, world!");
    
    {
        let mut s: String = String::from("hello"); // 从此处起 s有效
        
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
}
