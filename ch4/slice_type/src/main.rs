#![allow(unused)]

fn main() {
    // * s is Reference to the entire string
    let s = String::from("Hello, World");

    let result = first_word(&s);

    println!("{result}");

    // * Slice is reference to a portion of the string
    let hello = &s[0..5];
    let world = &s[7..12];

    let my_string_literal = "This is string literal";
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);
    let word = first_word(my_string_literal);

    // * Other slices
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    let arr_slice = &arr[1..3];
    assert_eq!(arr_slice, &[2, 3])
}

// * return a slice, not string
fn first_word(str: &str) -> &str {
    let str_in_bytes = str.as_bytes();

    for (index, &value) in str_in_bytes.iter().enumerate() {
        if value == b' ' {
            return &str[0..index];
        }
    }
    return &str[..];
}
