use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

// Box<dyn Error>> 意味着函数会返回一个实现Error trait的类型，但我们并不需要指定具体的类型是什么
// 这意味着我们可以在不同的错误场景下返回不同的错误类型
// 语句中的dyn关键字所表达的正式这种 “动态” (dynamic) 的含义
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    print!("With text :\n{}", content);
    Ok(())
}
