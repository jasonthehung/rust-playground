fn main() {
    // ! dangling problem
    /*
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
    */

    let string1 = String::from("Jason");
    let string2 = String::from("Wang");

    let result = get_longest(&string1, string2.as_str());

    println!("The longest string is: {}", result);
}

fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
