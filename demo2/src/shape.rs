// 定义一个 Shape trait，其中包含一个用于计算面积的 area 方法
pub trait Shape {
    fn area(&self) -> f64;
}

// 为圆形实现 Shape trait
pub struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 为三角形实现 Shape trait
pub struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 为正方形实现 Shape trait
pub struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 使用泛型和泛型约束来定义一个函数，该函数接受一个实现了 Shape trait 的类型作为参数
pub fn print_area<T: Shape>(shape: T) {
    println!("The area of the shape is: {}", shape.area());
}

// 主函数，用于测试
pub fn test() {
    let circle = Circle { radius: 5.0 };
    let triangle = Triangle { base: 5.0, height: 10.0 };
    let square = Square { side: 4.0 };

    print_area(circle);
    print_area(triangle);
    print_area(square);
}
