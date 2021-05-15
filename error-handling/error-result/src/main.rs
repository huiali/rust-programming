// Rust 将错误组合成两个主要类别：可恢复错误（recoverable）和 不可恢复错误（unrecoverable）。
// 可恢复错误通常代表向用户报告错误和重试操作是合理的情况，比如未找到文件。
// 不可恢复错误通常是 bug 的同义词，比如尝试访问超过数组结尾的位置。

// Rust 的错误处理功能被设计为帮助你编写更加健壮的代码。panic! 宏代表一个程序无法处理的状态，
// 并停止执行而不是使用无效或不正确的值继续处理。
// Rust 类型系统的 Result 枚举代表操作可能会在一种可以恢复的情况下失败。
// 可以使用 Result 来告诉代码调用者他需要处理潜在的成功或失败。
// 在适当的场景使用 panic! 和 Result 将会使你的代码在面对不可避免的错误时显得更加可靠。

use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error)
    //     },
    // };

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // 使用闭包实现

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // 失败时 panic 的简写：unwrap 和 expect
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// 传播错误

fn read_username_from_file() -> Result<String, std::io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 传播错误的简写：? 运算符
fn read_username_from_file2() -> Result<String, std::io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)


    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    // ? 运算符可被用于返回 Result 的函数
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// 当你期望在不返回 Result 的函数中调用其他返回 Result 的函数时使用 ? 的话，有两种方法修复这个问题。
// 一种技巧是将函数返回值类型修改为 Result<T, E>，如果没有其它限制阻止你这么做的话。
// 另一种技巧是通过合适的方法使用 match 或 Result 的方法之一来处理 Result<T, E>。

// main 函数是特殊的，其必须返回什么类型是有限制的。main 函数的一个有效的返回值是 ()，同时出于方便，另一个有效的返回值是 Result<T, E>

fn main1() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
// Box<dyn Error> 被称为 “trait 对象”（“trait object”）