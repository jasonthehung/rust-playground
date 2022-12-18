use std::{collections::HashMap, hash::Hash};
use unicode_segmentation::UnicodeSegmentation;
#[allow(unused)]

fn main() {
    // * 這是一個array
    let a = [1, 2, 3];

    // * 一個空的i32 vector
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // * vec macro 幫我們產生vector順便放值進去
    let mut v2 = vec![1, 2, 3];

    let third = &v2[2];
    // let third = &v2[100]; // ! runtime error
    println!("The third elements is {}", third);

    // @如果用index去access vector中的element, 有可能會有runtime error
    // # 所以更好的做法是用 get function
    match v.get(2) {
        Some(third) => println!("The third elements is {}", third),
        None => println!("There is no third element."),
    }

    // * Loop through all of elements
    let mut v3 = vec![1, 2, 3, 4, 5];
    for i in &mut v3 {
        *i += 50; // @ dereference
    }

    for i in &v3 {
        println!("{}", i); // @ v3 = [51, 52, 53, 54, 55]
    }

    // @ 把enum存到vertor裡面, 如次一來vector就可以存不同型態的資料了
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    match row.get(10) {
        Some(e) => match e {
            SpreadsheetCell::Int(i) => println!("This is a integer : {}", i),
            SpreadsheetCell::Float(i) => println!("This is a float : {}", i),
            SpreadsheetCell::Text(i) => println!("This is a string : {}", i),
        },
        None => println!("There is no element."),
    }

    // * Strings are stored as a collection of UTF-8 encoded bytes
    // * Unicode當中的前128個symbol, 其實就是ASCII
    // @ string slice並沒有該string的所有權 (it doesn't own the string, just points to it)
    let s1 = String::new(); // # 產生一個empty own string
    let s2 = "initial contents"; // # 這是一個string slice
    let s3 = s2.to_string(); // # 把string slice轉成own string
    let s4 = String::from("initial contents"); // # 直接把string slice變成own string

    let mut s5 = String::from("hello");
    s5.push('c');
    s5.push_str("world");
    println!("s5 is : {}", s5);

    let s6 = s5 + &s4; // ! s5已經被borrow給s6了
    println!("s6 is : {}", s6);

    let s7 = String::from("中文字");
    // # s7 in bytes [228 184 173 230 150 135 229 173 151]
    for b in s7.as_bytes() {
        print!("{} ", b);
    }

    // # s7 in chars [中 文 字]
    for c in s7.chars() {
        print!("{} ", c);
    }

    // # s7 in Grapheme clusters [中 文 字]
    // ! 標準函式庫沒有這個功能, 所以要額外使用crates來完成
    for g in s7.graphemes(true) {
        print!("{} ", g)
    }

    // * HashMap Section starts here
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();
    // ! 此時blue & yellow value的owner已經被轉移了, 所以時候要去存取blue or yellow會失敗
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    // ! 為什麼這邊是回傳Option? / 因為不能保證回傳會是什麼, 假如給了一個invalid key, 則會回傳None
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("Key: {} and Value: {}", key, value);
    }

    // * HashMap簡單的範例 (分析多個字串出現的次數, 並記錄在HashMap中)
    let text = "Jason Kevin Jason Bella";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // @ 如果該key目前不存在, value就預設給0; 假如key存在, 那就把原本的value deference然後 +1
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
