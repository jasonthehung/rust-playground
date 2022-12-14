#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(unused)]
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // @ Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct.
    fn squre(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rec2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rec3 = Rectangle {
        width: 60,
        height: 45,
    };

    let sq = Rectangle::squre(3);

    println!("The area of the rectangle is {} squre pixels.", rec1.area());
    println!("The rectangle has a nonzero width; it is {}", rec1.width());
    println!("Can rect1 hold rect2? {}", rec1.can_hold(&rec2));
    println!("Can rect1 hold rect3? {}", rec1.can_hold(&rec3));
    println!("Sq is : {}", sq.area());
}
