#![allow(unused)]

fn main() {
    println!("Hello, world!");

    let x = {
        let y = 1;
        y + 1
    };

    println!("x is: {x}");

    let five = five();
    println!("Calling function five is : {five}");

    other_function(5);
}

fn other_function(x: u32) {
    println!("Jason's here {x}");
}

fn five() -> i32 {
    -5
}
