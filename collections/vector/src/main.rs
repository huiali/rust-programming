fn main() {
    // 新建 vector
    let v: Vec<i32> = Vec::new();

    // 为了方便 Rust 提供了 vec! 宏
    let v = vec![1, 2, 3];

    // 更新 vector

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // vector 在其离开作用域时会被释放
    {
        let v = vec![1, 2, 3, 4];
        // 处理变量 v
    } // <- 这里 v 离开作用域并被丢弃

    // 读取 vector 的元素
    // 访问 vector 中一个值的两种方式，索引语法或者 get 方法：
    // 这两个不同的获取第三个元素的方式分别为：使用 & 和 [] 返回一个引用；或者使用 get 方法以索引作为参数来返回一个 Option<&T>。
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 一旦程序获取了一个有效的引用，借用检查器将会执行所有权和借用规则来确保 vector 内容的这个引用和任何其他引用保持有效。
    // 不能在相同作用域中同时存在可变和不可变引用的规则。
    // 当我们获取了 vector 的第一个元素的不可变引用并尝试在 vector 末尾增加一个元素的时候，这是行不通的：
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    // v.push(6);
    // cannot borrow `v` as mutable because it is also borrowed as immutable
    // mutable borrow occurs hererustc(E0502)

    println!("The first element is: {}", first);

    // 遍历 vector 中的元素
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // 遍历 vector 中元素的可变引用
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // 使用枚举来储存多种类型
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
