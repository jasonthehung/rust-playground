use std::env;
use std::process;

use io_project::Config;

fn main() {
    // # 在一開始執行程式前, 可以給傳入的參數
    // * cargo run Jason Wang -> ["target/debug/IO_project", "Jason", "Wang"]
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    if let Err(e) = io_project::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
