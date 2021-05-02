fn main() {
    println!("Hello, world!");
    another_function_1();
    another_function_2(3);

    another_function_3(1, 2);

    // 包含语句和表达式的函数体
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    //调用带有返回值的函数
    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn another_function_1() {
    println!("Another function.");
}
fn another_function_2(x: i32) {
    println!("The value of x is: {}", x);
}

// 必须 声明每个参数的类型
fn another_function_3(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// 具有返回值的函数
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
