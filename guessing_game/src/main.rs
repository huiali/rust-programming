use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Rng 是一个 trait，它定义了随机数生成器应实现的方法，想使用这些方法的话，此 trait 必须在作用域中。
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // loop 关键字创建了一个无限循环。将其加入后，用户可以反复猜测：
    loop {
        //使用变量储存值
        let mut guess = String::new();

        // & 表示这个参数是一个 引用（reference），它允许多处代码访问同一处数据，
        // 而无需在内存中多次拷贝。引用是一个复杂的特性，Rust 的一个主要优势就是安全而简单的操纵引用。
        // 完成当前程序并不需要了解如此多细节。现在，我们只需知道它像变量一样，默认是不可变的。
        // 因此，需要写成 &mut guess 来使其可变，而不是 &guess。（第四章会更全面的解释引用。）
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // io::Result   使用 Result 类型来处理潜在的错误
        // Result 的成员是 Ok 和 Err，Ok 成员表示操作成功，内部包含成功时产生的值。Err 成员则意味着操作失败，并且包含失败的前因后果。
        // Rust 允许用一个新值来 隐藏 （shadow） guess 之前的值。这个功能常用在需要转换值类型之类的场景
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
