use add_one;
use add_two;

// ! unresolved import `rand`  no external crate `rand`
// use rand;

fn main() {
    let num: u32 = 10;
    println!("{}", add_one::add_one(num));

    println!("{}", add_two::add_two(num));
}
