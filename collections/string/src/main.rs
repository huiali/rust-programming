fn main() {
    // 新建字符串
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // 该方法也可直接用于字符串字面值：
    let s = "initial contents".to_string();

    // 更新字符串
    // 使用 push_str 和 push 附加字符串
    let mut s = String::from("foo");
    s.push_str("bar");

    // push_str 方法采用字符串 slice，因为我们并不需要获取参数的所有权。
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);

    // push 方法被定义为获取一个单独的字符作为参数，并附加到 String 中

    let mut s = String::from("lo");
    s.push('l');

    // 使用 + 运算符或 format! 宏拼接字符串

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

    //println!("s1 is {}", s1); //borrow of moved value: `s1`
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);
    println!("s is {}", s);

    // 索引字符串
    let s1 = String::from("hello");
    // let h = s1[0]; // Rust 的字符串不支持索引。

    //字符串 slice
    let s1 = String::from("hello");
    let h = &s1[0..];

    let hello = "Здравствуйте";
    let s = &hello[0..4];

    // 遍历字符串的方法
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // bytes 方法返回每一个原始字节
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
