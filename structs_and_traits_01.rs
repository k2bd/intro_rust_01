struct Circle {
    radius: f64,
}

struct Rectangle {
    a: f64,
    b: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.a * self.b
    }
    fn near_square(&self, tol: f64) -> bool {
        (self.a / self.b - (1 as f64)).abs() < tol
    }
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}


fn main() {
    let circle = Circle{radius: 5.0};

    let square = Rectangle{a: 2.0, b: 3.0};

    let is_larger = circle.area() > square.area();
    println!("Circle is larger than square??? {}", is_larger);

    println!("Rectangle is nearly square??? {}", square.near_square(0.0001));
}
