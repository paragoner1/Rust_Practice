#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
    fn square(size: u32) -> Rectangle {  // Associated function—no &self
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle::square(20);  // Call with ::, not .
    let rect3 = Rectangle { width: 60, height: 45 };
    
    println!("rect1: {:?}", rect1);
    println!("rect2 (square): {:?}", rect2);
    println!("Area of rect1: {}", rect1.area());
    println!("Perimeter of rect2: {}", rect2.perimeter());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
}