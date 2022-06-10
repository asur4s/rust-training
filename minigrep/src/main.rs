use minigrep::Config;
use std::{env, process};

fn main() {
    // env::args 会返回一个迭代器，迭代器会将参数以 OSString 类型返回，
    // 使用 collect 可以将迭代器转换为数组。
    let args: Vec<String> = env::args().collect();

    // 如果返回 Options 类型，就执行 unwrap 展开，否则展开后执行闭包的内容。
    // 闭包接收 Err 中的字符串，然后打印，并退出。
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
