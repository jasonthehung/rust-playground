#![allow(unused)]
use std::io::{self, Read};

fn main() {
    let mut p1 = Point { x: 1, y: 2 };
    let mut p2 = Point { x: 'J', y: 'H' };

    let p3 = p1.mixup(p2);

    println!("x: {}, y: {}", p3.x, p3.y);

    let mut res = String::new();
    let a = io::stdin().read_line(&mut res);
    println!("Input {} contains: {:?} byte", res, a);

    let integer = Option::Some(5);
    let float = Option::Some(5.00);
}

// @ Generic Enum可以解決重複的問題
enum Option<T> {
    Some(T),
    None,
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
