/*
描述：
    定义一个枚举Shape，表示两种形状：Circle（圆形）和Rectangle（矩形）。
        每种形状都有相应的属性，如圆形有radius（半径），矩形有length和width（长和宽）。
    然后，为该枚举实现一个方法area()，返回每个形状的面积。

要求：
    使用枚举表示不同的形状。
    每个变体存储与之相关的值（Circle存储radius，Rectangle存储width和height）。
    为Shape枚举实现area()方法，计算并返回形状的面积。
    在main函数中创建不同的Shape实例并计算它们的面积。
*/

/*
知识点：
    1、enum嵌套struct
    2、match 的使用，注意结构体的引用
    3、枚举嵌套结构体声明
*/

struct MyCircle {
    radius: f64,
}

struct MyRectangle {
    length: f64,
    width: f64,
}

enum Shape {
    Circle(MyCircle),
    Rectangle(MyRectangle),
}

impl Shape {
    fn area (&self) -> f64 {
        match self {
            Shape::Circle(c ) => { // match 匹配 枚举成员，若匹配成功，将self中的Circle中的my_circle借用给c（是引用，不是赋值，默认推导出来的）
                c.radius * c.radius * 3.14
            },
            Shape::Rectangle(r) => {
                r.length * r.width
            },
        }
    }
}
fn main(){
    let wpx_shape = Shape::Circle(
        MyCircle {
            radius: 2.0,
        }
    ); // 自研 枚举嵌套结构体声明

    println!("area = {}", wpx_shape.area());

    let wpx_rectangle = Shape::Rectangle(
        MyRectangle {
            length: 1.0,
            width: 1.0,
        }
    );
    println!("area = {}", wpx_rectangle.area());
}
