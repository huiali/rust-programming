// *^ 引用及借用

fn main() {
    let s1 = String::from("hello");

    // &s1 语法让我们创建一个 指向 值 s1 的引用，但是并不拥有它。
    // 因为并不拥有这个值，当引用离开作用域时其指向的值也不会被丢弃。
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

//s 引用 s1的指针，允许你使用值但不获取其所有权，&String s 指向 String s1
fn calculate_length(s: &String) -> usize {
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
  // 所以什么也不会发生

//   我们将获取引用作为函数参数称为 //* 借用（borrowing）

// * 可变引用

fn main1() {
    let mut s = String::from("hello");
    change(&mut s);
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
// 可变引用有一个很大的限制：在特定作用域中的特定数据只能有一个可变引用。
// 这个限制的好处是 Rust 可以在编译时就避免数据竞争。
// 数据竞争（data race）类似于竞态条件，它可由这三个行为造成：
// 1'两个或更多指针同时访问同一数据。
// 2'至少有一个指针被用来写入数据。
// 3'没有同步数据访问的机制。

// let mut s = String::from("hello");
// {
//     let r1 = &mut s;
// } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
// let r2 = &mut s;

// 类似的规则也存在于同时使用可变与不可变引用中。

// let mut s = String::from("hello");

// let r1 = &s; // 没问题
// let r2 = &s; // 没问题
// let r3 = &mut s; // 大问题

// println!("{}, {}, and {}", r1, r2, r3);

// * 悬垂引用（Dangling References）
// 在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针（dangling pointer），
// 所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。
// 相比之下，在 Rust 中编译器确保引用永远也不会变成悬垂状态：当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。

fn main2() {
    let reference_to_nothing = dangle();
}

// missing lifetime specifier.
fn dangle() -> &String {
    // dangle 返回一个字符串的引用
    let s = String::from("hello"); // s 是一个新字符串
    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 危险！

// * 引用的规则
// 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
// 引用必须总是有效的。
