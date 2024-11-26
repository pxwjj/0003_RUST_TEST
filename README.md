# the book 学习

1. rust是一个基于**表达式**的语言

## 1 常量

- 常量只能被设置为常量表达式， 而不可以是其他任何只能在运行时计算出来的值。
  - `const THREE_HOURS_IN_SECONDS : u32 = 3 * 60 * 60`
- 命名
  - 单词 + “_”

## 2 隐藏

- 定义一个与之前变量**同名**的新变量
  - 第二个变量 被 第一个变量隐藏了 （shadowing），在后续使用该名称时候，编译器将看到 第二个变量
  - 可以改变类型
    - mut 不可以该变类型

## 3 数据类型

- 默认会推导数据类型，但有时是必要添加 数据类型的
  - eg：`let w_num: u32 = match "4".parse().expect("string to num err!");`
    - 这里 w_num 需要 指定 :u32 类型

## 4 标量类型

- 整型 默认为i32
  - 有符号
    - 有符号 类型，以**补码**形式存储
    - [-(2 ^ n-1)] ~ [(2 ^ n-1)-1]
  - 无符号
    - [0] ~ [(2^n) - 1]
  - 0x（16进制）   0o（8进制）    0b（2进制）
  - **整型溢出**
    - debug：编译后，整型溢出会导致程序panic
    - release：编译后，超出最大值后会回绕到最小值
- 浮点型 默认f64
  - f32（单精度）
  - f64（双精度）
- 布尔型
  - true
  - false
- 字符类型 - 4字节
  - 'z' 
  - 'c'

## 5 复合类型

- 元组 - tuple
  - `  let t: (i32, f64, bool) = (2, 0.1, true);`
  - 解构元组： `  let (x, y, z) = t;` // x=2   y=0.1  z=true 
  - 索引访问：`println!("t.0 = {}, t.1 = {}, t.2 = {}", t.0, t.1, t.2);`  // 用 (.) 来索引 tuple，第一个索引值是 .0
  - 不带任何值的元组，有一个特殊的名字，叫 unit，这种值的类型都写作()，表示空值或空的返回类型。
- 数组 - array
  - 长度是固定的
  - 类型是相同的
  - 将数组的值写在方括号内，用逗号分开。
  - `  let a: [i32; 4] = [1, 2, 3, 4];` // [i32 ; 4]
  - 确定个数，用数组，如月份，不确定个数，用vector。
  - 数组是在栈上，分了固定的长度，来存内容，用索引来取元素值。用 ([])来索引数组
  - `println!("a[0] = {}", a[0]);` // a[0]=1
  - 在使用索引时，rust会检测当前index是否可一索引，若超出数组索引范围，则程序会painc，**而不是允许继续访问一块没有使用的内存**。

## 6 函数

### 6.1 代码风格

- 使用 snake case 的命名规则
  - 全小写，多个单词之间用“_”连接
- 函数可以定义在调用者之前以及之后，**不用说是，必须在调用者之前来定义此函数**。只要函数在调用者相同的作用域中即可。
- 在函数的声明中，必须指明参数的类型，定义多个参数时用”,“分开
  - 这是rust在设计中 经过慎重考虑的决定。
- 函数的返回值，不需要命名，但是需要指定类型。（->）
  - **函数的返回值，等同于函数体中最后一个表达式的值。可以使用return ，但是大部分函数隐式返回最后的表达式。**

```rust
fn main () {
    print_hour(2, 'h');
    five(4);
}

fn five(num: u32) -> u32 { //指明返回值类型
    5 //不要加分号，加了就是语句，不是表达式了。
} 

// 就算在main函数下方定义，main函数也能调用
fn print_hour (num: u32, ocloc: char) {  // 参数需要指明类型
    println!("how time? {num}{ocloc}");
}
```

### 6.2 语句

- 执行一些操作，但不返回值
- 语句没有返回值，使用单位类型**()** 来表示不返回值

```rust
fn main (){  // 这玩意 也是语句，声明函数的语句
	let x = 2; //这玩意就是 语句，只要不返回值，就是 语句

	let y = (let a = 1); // 报错，因为 let a = 1 没有返回值，怎么能赋值给 y呢
}
// 不能像c那样 x = y = 10
```

### 6.3 表达式

- 计算并产生一个值
- 大括号创建一个新的块作用域，也是一个表达式，结尾没有分号，有的话，就不是表达式了，而是语句。

```rust
// 说白了，5+6 是一个表达式
// 表达式的结尾没有分号

fn main () {
    let x = { 	// 大括号创建的一个新的块作用域 也是一个表达式。 注意 结尾没有分号
        let y = 1;
        y + 1 
    };
}
```

## 7 流程控制

### 7.1 if

- 条件必须为bool值

```rust
	let mut a = 5;
	if a { // 程序编译不过。条件必须为bool值才行
        println!("a 为 bool 值"); 
    }
```

- 不像其他语言会将 一些非布尔类型的值，转为为 bool类型的值。 

### 7.2 loop

- continue
- break ， 有时后面也能跟表达式，跟return用法相似。
- 循环标签，在多个循环嵌套时候，消除歧义。

```rust
    let mut count: u32 = 0;
    'counting_up: loop {  // 引号
        println!("count = {count}");
        
        let mut count_1: u32 = 0;
        loop {
            println!("count_1 = {count_1}");
            count_1 += 1;

            if count_1 == 3 {
                break; // 跳出当前循环
            }else if count == 2 {
                break 'counting_up; // 注意用法 。跳出 'counting_up 标签所在的循环体
            }
        }
        count += 1;
    }
```

### 7.3 while

- 条件也是 bool 值

```rust
    let mut while_num: u32 = 10;
    while while_num > 0 {
        println!("while_num = {while_num}");

        while_num -= 1;
    }
```

### 7.4 for

- 遍历集合（数组）
- for x in array {}   // 在array中取值，赋值给x

```rust
    // for 
    let array_for: [i32; 5] = [1, 2, 3, 4, 5];
    for ele in array_for {
        println!("ele = {ele}");
    }

    for ele in (1..=4).rev() { // rev() 反转
        println!("ele = {ele}");
    }
```

## 8 所有权（ownership）

- 是rust 管理内存的一组规则
- 要处理的问题：
  1. 跟踪哪部分代码正在使用堆上的数据
  2. 最大限度的减少堆上重复数据的使用
  3. 清理堆上不需要使用的数据，确保不会耗尽内存空间。
- **所有权最主要的目的就是管理堆数据**。
- 当持有堆中数据的变量，在离开作用域时候，要被调用drop清理掉，除非数据被move至另一个变量所有

### 8.1 一共三种垃圾回收机制（GC），包含其他语言

- 程序运行时有规律的寻找不再使用的内存
- 程序员亲自释放
- rust使用第三种方式：通过所有权系统管理内存，编译器在编译时，会根据一系列规则进行检查。
  - 在运行时所有权系统的任何功能都不会减慢程序
  - 违反了这些规则，程序编译不通过。（感觉因为是编译时检查，所以运行时候，不会减慢程序）

### 8.2 堆

- 在编译时，大小未知或大小可能变换的数据，要改为存储在堆上。
- 访问堆上的数据，比访问栈上的数据慢： 因为必须通过指针来访问。

### 8.3 栈

- 栈中的所有数据都必须占用已知且固定的大小。

### 8.4 所有权规则

1. rust中每个值都有一个**所有者**
2. 值在任何时刻**有且只有**一个所有者
3. 当所有者（**变量**）离开作用域时，这个值将被丢弃

### 8.5 变量作用域

- 作用域（scope）是一个项（item）在程序中有效的范围。

```rust
fn main() {
    println!("Hello, world!");
    
    {
        let mut s: String = String::from("hello"); // 从此处起 s有效
        
        // 使用s 
    }   
    // s不在有效 默认调用了 drop 函数
}
```

### 8.6 变量与数据的交互方式

#### 8.6.1 移动

```rust
fn main() {
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
}
```

#### 8.6.2 克隆

- 深拷贝，将堆上的数据，再拷贝一份，源数据依旧存在
- 当出现clone调用时，程序员会知道一定的代码被执行，并且，这段代码可能会消耗资源。

```rust
    {
        let s1: String = String::from("value");
        let s2 = String::clone(&s1); // clone 深拷贝

        println!("{s1} {s2}");
    } // 会drop s1 和 s2
```

### 8.7 所有权与函数

```rust
fn main(){
    // 所有权与函数
    let s1: String = String::from("value");
    test_ownership(s1); // s1的所有权给something , 调用完此函数之后，不能再使用s1
    // println!("s1 = {s1}"); // 报错

    let a: u32 = 1;
    test_u32_cope(a); // 因为u32为cope类型，所以，a还能使用。并不是把a的所有权交给num。
    println!("a = {a}");
}

fn test_ownership(something: String){
    println!("something = {something}");
} // 调用drop somtthing 占用的内存被释放

fn test_u32_cope(num: u32){
    println!("num = {num}");
}
```

- 变量进入函数中，不希望将所有权也给入参，还希望入参能获取到值。涉及到**引用**

## 9 引用 - 可变 - 垂悬

- 不用获取所有权就可以使用值的功能
- 将创建引用的行为成为**借用**
- **可变引用**才可以修改默认值
  - **不能有两个可变引用，同时作用在一个变量上。**避免数据竞争
  - **不能在有不可变引用的时候，再增加一个可变引用。**不可变引用的使用者，不希望，值莫名其妙得被改变了。
- 引用的规则：
  1. 在任意给定时间，要么只能有一个可变引用，要么只能由多个不可变引用。
  2. 引用必须总是有效的

```rust
fn change_str(str: &mut String){ // 入参为可变引用
    str.push_str(" string");
}

fn main() {
    let mut s1: String = String::from("value"); // 可变字符串
    change_str(&mut s1); // 可变引用
    println!("change after {s1}");
}
```

### 9.1 悬垂引用

- **释放内存时，保留指向它的引用**
  - 也就是说，主人把东西借给了别人，然后又把东西销毁了。【会编译报错】

```rust
let refrence_str = ref_dangle();

fn  ref_dangle() -> &String {
    let s: String = String::from("value");
    &s
} // s借用给了，refrence_str。 但是出了作用域已经被drop，此时refrence_str是一个悬垂引用
```

## 10 slice

- **集合**中，**连续**的元素
- 是一种**引用**，**没有所有权**
- 字符串字面值是一个slice
  - `let a: &str = "hello";`
- &str 是**不可变引用**，所有，字符串字面值是不可变的

```rust
fn first_word_index(s: &String) -> usize {
    let bytes_array = s.as_bytes();
    for (i, &item) in bytes_array.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}

// 该函数接收一个用空格分隔单词的字符串， 返回该字符串中的第一个单词。
fn first_word(s: &String) -> &str { // 传引用，返回切片（一个字符串中的一部分）
    let bytes_array = s.as_bytes(); // 当作字节数组，然后再操作
    for (i,&item) in bytes_array.iter().enumerate() { // 用数组迭代器，并且返回index 和 指针。迭代器返回的是引用
        if item == b' ' { // b' ' 表示空格的字节值
            return &s[..i]; // 第i位为空格， &s[0..10] 不包含index 为10的位置
        }
    }
    return &s[..]; 
}

// 更加通用，入参可以传切片
fn first_word_good(s: &str) -> &str {
    let bytes_array = s.as_bytes();
    for (i, &item) in bytes_array.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

fn main() {
    let s_test: String = String::from("pxwjj wrj");
    let first_string = first_word(&s_test);
    
    println!("{first_string}");
    let slice_test1: &str = &s_test[..];
}
```

---

## 11 小结

- 所有权，借用，slice让rust程序在编译时，确保内存安全，
- rust语言提供了与其他系统编程语言相同的方式来控制你使用的内存，但**所有者**在离开作用域后自动清除释放，意味着程序员无需额外的编写释放内存的代码

## 12 结构体 - struct

| 元组                     | 结构体                           |
| ------------------------ | -------------------------------- |
| 每一部分可以是不同类型   | 每一部分可以是不同类型           |
| 每一部分不用命名         | 每一部分需要命名                 |
| 依赖顺序来访问元组中的值 | 不需要依赖顺序访问结构体中的实例 |

- 结构体中每一部分，称为**字段**
- 创建一个实例，需要用结构体名称开头，后面大括号中，用key:value的形式赋值，**每个实例字段中间用逗号隔开**
- 用**点号**获取结构体中的某个字段的值，如果结构体是可变得，也可以用点号赋值
  - 不允许只将某个字段标记为可变
- 结构体也可以作为一个函数的返回值（结构体就相当于是一个类型，理所应当可以被函数返回）

```rust
struct User {
    name: String,
    age: u32,
    email: String,
    active: bool,
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

    let mut wrj: User = User { // 可变结构体
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
}
```

- 结构体并没有提供一个Display实现来使用println!与{}占位符。
  - println! 使用Display的格式。基本类型都默认实现了Display，所有，println!能通过{}打印出来变量值
  - 打印结构体，需要在结构体上方，添加宏，和使用{:?}

```rust
#[derive(Debug)] // 不添加此宏，println!打印不出来结构体
struct Rectangle {
    length: u32, 
    width: u32,
}
fn main() {
    let R: Rectangle = Rectangle {
        length: 2,
        width: 10,
    };
    // 查看结构体实例的值
    println!("{:?}", R); // 在一行将结构体所有字段打印出来
    println!("{:#?}", R); // 打印的更漂亮，分多行
} 
```

## 13 方法

### 13.1 和函数比较

|      | 方法                     | 函数               |
| ---- | ------------------------ | ------------------ |
| 相同 | fn关键字和名称声明       | fn关键字和名称声明 |
| 相同 | 拥有入参和返回值         | 拥有入参和返回值   |
| 相同 | 包含函数体               | 包含函数体         |
| 不同 | **在结构体上下文中定义** | **在任何位置定义** |
| 不同 | **第一个入参总是self**   | **入参没有要求**   |

```rust
use rand::Rng;

// 每次看一下自己的账户，钱都会增加
#[derive(Debug)]
struct bank_user {
    name: String,
    age: u32,
    money: u32,
}

// 使用impl关键字
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
    //--
    let mut wpx = bank_user {
        name: String::from("wpx"),
        age: 24, 
        money: rand::thread_rng().gen_range(50000..=100000),
    };
    
    println!("money = {}", bank_user::money_add(&mut wpx));
    wpx.get_money();
	
    //--
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
	
    //--
    let test: bank_user = bank_user::init_money(10000);
    println!("{:#?}", test);
}
```

### 13.2 关联函数

- 上面代码中展示 init_money方法

## 14 枚举 - enum

- 理解为：要么是这个值，要么是哪个值
- 有著名的Option枚举，要么是一个值，要么是空值（None）
  - 有值的话，实例化时不用写类型，可以推导出。
  - 没有值的话，实例化时候必须写明类型。

![image-20241113221052153](F:\学习笔记\rust学习笔记\thebook学习.assets\image-20241113221052153.png)

```rust
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
}
```

## 15 match

### 15.1 github 

- https://github.com/pxwjj/0003_RUST_TEST/blob/thebook_branch/enum_test/src/gongsi_test1.rs

  - enum嵌套结构体，随后match。

  -     1、enum嵌套struct
        2、match 的使用，注意结构体的引用
        3、枚举嵌套结构体声明

- 想要不管任何情况都进入，需要使用通配符 '_'或者“other” 来进行匹配

- 如果什么也不想操作的话，使用 `=> ()`

- 注意`#[derive(Debug)]`宏的使用。要搭配 `{:#?}`

```rust
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
    println!("{a:#?}"); //更好看一点
    match_student(&a);
}
```

## 16 包、Crate、模块管理

- **crate**两种形式
  - 二进制项
  - 库
- 一个**包**中包含
  - 最多一个 库crate (library crate)
  - 可以有多个 二进制crate(binary crate)
- 当实现了一个操作后，其他代码可以调用改代码的**公共接口**
  - 公共部分
  - 私有部分
- crate是rust再编译时最小的代码单位
- 这部分最主要理解模块系统

### 16.1 定义模块 - 有规则

#### 一个模块

1. 从crate根节点开始
   - 在**根文件**中寻找需要被编译的代码
     - 二进制crate而言，根文件就是 main.rs
     - 库crate而言，根文件是lib.rs
2. 声明模块
   - 在crate根文件中声明 `mod people; // 声明了一个people模块`
   - 编译器会在 大括号中（内联）寻找模块代码
3. 在文件 `src/people.rs`
4. 在文件 `src/people/mod.rs`
5. 声明子模块
   - 在 people.rs中使用语句`mod student;` 定义了 people模块下的student 子模块
6. 在文件 `src/people/student.rs`
7. 在文件 `src/people/student/mod.rs`
8. 模块中的代码路径
   - 一旦一个模块是crate中的一部分，那就可以在隐私规则允许的前提下进行调用
     - 比如：调用people模块下的student模块下的study类型，`crate::people::student::study`
9. 私有vs公有
   - 一个模块中的代码默认对其父模块**私有**
   - 为了使一个模块公用，在声明时使用`pub mod`
10. use关键字
    - 参照第八点，使用类型的时候，太长了，使用use后，就可以直接使用 study类型

#### 举例并说明相关知识点

创建一个库crate `cargo new --lib all` 

- 绝对路径
- 相对路径
- 使用pub
  - **模块公有并不会使内容公有** 因此该函数也需要pub关键字
  - 结构体也一样
    - 将结构体设置为公有，其成员也需要设置公有才可以访问
  - enum则不同
    - 将enum设置为公有，其成员不需要设置pub关键字就可以访问
- 使用super
  - 在子模块中使用
  - 可以让子模块使用**父模块（仅限父模块，爷爷模块可不行）**中的内容，而不用从crate开始写模块路径
- 使用use 
  - **注意，标准库对你的包来说，也是一个外部crate**

```rust
mod people {
    fn have_name(){}

    pub mod student {
        pub fn get_student_name(){}
        fn get_student_age(){}
        mod daxue {
            fn get_daxue_name(){
                // use crate::people::student;
                // student::get_student_age();

                super::get_student_name(); // 通过super 使用父模块中的接口
            }
        }
    }

    mod worker {
        fn get_worker_name(){
            super::have_name(); // 通过super 使用父模块中的接口
        }
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
```

```rust
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
```

### 16.2 将模块拆成多个文件

- 把 back_of_house 大括号中的内容移到 back_of_house.rs文件中。

`lib.rs:`

```rust
mod back_of_house;

fn eat_at_restaurant(){

    let mut meal = back_of_house::Breakfast::summer(String::from("Rye")); // 将结构体变为pub，对应的summer变为pub

    meal.toast = String::from("Wheat"); // 更改结构体中参数 将结构体成员变为 pub -- 结构体共有不是说结构体所有成员都共有

    // meal.seasonal_fruit =  String::from("banana");// 不能编译，因为seasonal_fruit 不是共有成员
}
```

`back_of_house.rs:`

```rust
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
```

- 如果back_of_house还有子模块的话，那么就创建一个back_of_house文件夹，在文件夹下再创建一个子模块名称对应的.rs文件。

<img src="F:\学习笔记\rust学习笔记\thebook学习.assets\image-20241118134411557.png" alt="image-20241118134411557" style="zoom: 50%;" />

## 17 Vector

- Vec类型
- 类型注解 -> 泛型
- vec!宏初始化，push添加元素，使用get获取元素（好处：返回值为 Option，索引不存在的话返回 None），使用索引获取元素（坏处：索引不存在，会导致painc）

```rust
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
```

## 18 String

### init

```rust
fn test_string_init() {
    let s1 = String::from("value");
    let s2 = "value".to_string();
    // 以上两种 是同一种方式，看使用习惯
}
```

### push_str and push

```rust
fn test_string_modify() {
    let mut s2 = "value".to_string();
    s2.push_str(" a"); // 追加string , 不会获取所有权
    println!("{s2}");

    s2.push('a'); // 追加 char
    println!("{s2}");
}
```

### '+' and 'format!'

- **`format!` 相当好用**

```rust
fn test_string_puls() {
    let s1 = "s1".to_string();
    let s2 = "s2".to_string();

    let s3 = s1 + &s2; // 只能这样写，第一个移动，第二个引用。 与add函数入参有关
    // s1不能使用了
    println!("{}", &s3[0..]);
    println!("{}", &s3); // &str[0..] == &String
}

// 使用format 代替plus
fn test_string_format() {
    let s1 = "s1".to_string();
    let s2 = "s2".to_string();

    let s3 = format!("{s1} 杂七杂八 {s2}");
    println!("{s3}");
}
```

### index

- hah 是不是觉得要进行字符串索引了？
- **String 不支持索引**
  - 原因：String是一个`Vec<u8>`
  - **不是所有字符的UTF-8都只占一个字节。** 

```rust
fn test_string_index() {
    let hello = "こんにちは".to_string();
    // 不是每个字符都是占用一个字节，因此不能索引
    // hello[0]; //编译不通过
}
```

```rust
fn test_string_index() {
    let hello = "こんにちは".to_string();
    // 不是每个字符都是占用一个字节，因此不能索引
    // hello[0]; //编译不通过

    // 但是可以使用 slice
    println!("{}", &hello[0..=2]); // 返回 こ ，可以发现，日文的一个字符占用三字节

    // -- 
    for i in hello.chars() {// 分别将 こ ん に ち は 拆分出来，打印
        println!("{i}");
    }
    for i in hello.bytes() { // 这个会打印出来每个字节 ,一共打印 3*5 个字节
        println!("{i}");
    }
}
```

