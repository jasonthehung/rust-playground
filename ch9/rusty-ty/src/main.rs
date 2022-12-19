#![allow(unused)]
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello2.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    read_string_from_file();

    // * 介紹一些好用的Result Enum
    // # 目前f的型態是Result, 所以可以用一些Result的function
    // # 而f2透過unwrap function之後, 變成了File
    // let _f2 = File::open("hello2.txt").unwrap();
}

fn read_string_from_file() -> Result<String, io::Error> {
    // @ 加一個問號在後面可以簡化Error Handling的檢查
    let mut f = File::open("hello.txt")?;

    /*
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
    */

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => {
            println!("{}", s);
            Ok(s)
        }
        Err(e) => Err(e),
    }
}
