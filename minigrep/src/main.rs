use std::env;
use std::process;

pub mod lib;
use crate::lib::run;
use crate::lib::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    //参数转换为对象
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments:{}", err);
        process::exit(1);
    });
    println!(
        "Searching for {} and in file {}",
        config.query, config.filename
    );

    //读取文件
    if let Err(err) = run(config) {
        println!("Application Error: {}", err);
        process::exit(1);
    }
}
