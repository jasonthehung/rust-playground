fn main() {
    let list1 = vec![1, 6, 3, 10, 7, -1, 7];

    let largest = get_largest(list1);

    println!("The largest value in list1 is: {}", largest);

    let list2: Vec<char> = vec!['a', 'c', 't', 'h', 's', 'q', 'z'];

    let something = get_largest(list2);

    println!("The largest value in list2 is: {}", something);
}

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];

    for each in number_list {
        if each > largest {
            largest = each;
        }
    }
    return largest;
}
