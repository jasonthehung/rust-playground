fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&mut s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s2: &mut String) -> usize {
    s2.push_str(", world");
    s2.len()
}
