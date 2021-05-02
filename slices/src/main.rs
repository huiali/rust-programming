fn main() {
    let mut s = String::from("hello world");

    let word = first_word1(&s); // word 的值为 5

    s.clear(); // 这清空了字符串，使其等于 ""
               // word 在此处的值仍然是 5，
               // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！

    //    字符串 slice
    // 字符串 slice（string slice）是 String 中一部分值的引用，它看起来像这样：
    let word2 = first_word2(&s);
    // 借用规则，当拥有某值的不可变引用时，就不能再获取一个可变引用。因为 clear 需要清空 String，它尝试获取一个可变引用。Rust不允许这样做，因而编译失败。
    // s.clear();
    println!("the word is {}", word2);

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("the first word is {}", hello);

    let s = String::from("hello");

    let slice1 = &s[0..2];
    let slice2 = &s[..2];

    println!("the slice1 is {},slice2 is {}", slice1, slice2);

    // 字符串字面值就是 slice
    let s = "Hello, world!";
}
fn first_word1(s: &String) -> usize {
    // 用 as_bytes 方法将 String 转化为字节数组
    let bytes = s.as_bytes();
    // 使用 iter 方法在字节数组上创建一个迭代器
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
