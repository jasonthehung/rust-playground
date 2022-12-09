#![allow(unused)]
fn main() {
    // * Immutable
    let y = 1;
    println!("The value of y is: {y}");
    // * Mutable
    let mut x = 5;

    // * Constants
    // ! (always annotate the type)
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // * Shadowing
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
        {
            let y = y * 2;
            println!("The value of y in the second inner scope is: {y}");
        }
    }

    let spaces = "   ";
    let spaces = spaces.len();
}
