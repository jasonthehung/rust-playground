fn main() {
    // * String literal vs. Mutable String
    // @ 主要差別在於使用Memory的方式

    // * Double free error
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}");

    let s = String::from("Batman");
    take_ownership(s);

    // * s strin已經被move掉了，所以這邊不能在印出s string了
    // println!("{}", s);

    let num = 5;
    makes_copy(num);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_number: i32) {
    println!("{}", some_number);
}
