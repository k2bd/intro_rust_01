// -------- Structs

struct Circle {
    radius: f64,
}

struct Rectangle {
    a: f64,
    b: f64,
}

impl Rectangle {
    fn near_square(&self, tol: f64) -> bool {
        (self.a / self.b - (1 as f64)).abs() < tol
    }
}


// -------- Traits

trait HasArea {
    fn area(&self) -> f64;

    fn is_larger(&self, other: &impl HasArea) -> bool {
        self.area() > other.area()
    }
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.a * self.b
    }
}


// -------- Main


fn main() {
    let circle = Circle{radius: 5.0};

    let square = Rectangle{a: 3.0, b: 3.0};

    println!("Circle is larger than square??? {}", circle.is_larger(&square));

    println!("Rectangle is nearly square??? {}", square.near_square(0.0001));
}
