fn main() {
    let mut p1: Point<u32, u32> = Point { x: 5, y: 10 };

    let mut p2: Point<char, String> = Point {
        x: 'J',
        y: String::from("Wang"),
    };

    // ! Overall: 想怎麼定義就怎麼定義～
}

#[derive(Debug)]
// @ 定義了一個名稱為Point的struct, 其中有兩個Generic types (T and U)
struct Point<T, U> {
    x: T,
    y: U,
}
