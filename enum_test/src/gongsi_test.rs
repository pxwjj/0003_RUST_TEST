/*
练习：
    定义一个TrafficLight枚举并模拟交通灯的变化
描述：
    定义一个TrafficLight枚举，表示交通信号灯的三种状态：Red（红灯）、Yellow（黄灯）、Green（绿灯）。
    然后，定义一个方法next()，用于根据当前交通信号灯的状态来决定下一个状态（即从Red到Green，从Green到Yellow，从Yellow到Red）。
要求：
    使用枚举类型TrafficLight表示不同的交通信号灯状态。
    为TrafficLight定义一个next()方法，返回下一个信号灯状态。
    在main函数中模拟一个循环，依次切换交通灯的状态，并打印每次的状态。
*/

/*
涉及到的知识点：
    1、enum的定义、实例化。enum理解为，要么是..要么是..
    2、enum中方法是定义、使用
    3、match 结合 enum 的用法
    4、enum结构的打印： #[derive(Debug)]  {:?}
    5、for循环的使用
*/

#[derive(Debug)] // 能打印出enum结构了
enum TrafficLight { // 三种变体
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn next(&self) -> TrafficLight {
        //call
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
        }
    }
}

fn main() {
    let mut my_light: TrafficLight = TrafficLight::Green;
    println!("初始状态为：{:?}", my_light);

    for i in 0..=6 { // 在0-6里面取值，取完后退出循环
        my_light = my_light.next();
        println!("第{}轮变化为{:?}", i, my_light);
    }
}
