use core::fmt::Display;

fn main() {
    {
        let r;                //-----------------+-- 'a
        {                     //                 |
            let x = 5;        //  -+--- 'b       |
            r = &x;           //   |             |
        }                     //  -+             |
        //println!("{}", r)     //               |
    }                         //-----------------|
    //借用检查器
    // 这里将 r 的生命周期标记为 'a 并将 x 的生命周期标记为 'b。
    // 内部的 'b 块要比外部的生命周期 'a 小得多。
    // 在编译时，Rust 比较这两个生命周期的大小，
    // 并发现 r 拥有生命周期 'a，不过它引用了一个拥有生命周期 'b 的对象。
    // 程序被拒绝编译，因为生命周期 'b 比生命周期 'a 要小：被引用的对象比它的引用者存在的时间更短。
    {
        let x = 5; //-----------------+-- 'b
                   //                 |
        let r = &x; //  -+--- 'a       |
                    //                 |
        println!("{}", r) //-----------------|
    }

    // 函数中的泛型生命周期
    let string1 = String::from("abcd");
    let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);

}

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }


// 生命周期注解语法
// &i32        // 引用
// &'a i32     // 带有显式生命周期的引用
// &'a mut i32 // 带有显式生命周期的可变引用

// 函数签名中的生命周期注解
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// 它的实际含义是 longest 函数返回的引用的生命周期与传入该函数的引用的生命周期的较小者一致。
// 泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个。
// 因为我们用相同的生命周期参数 'a 标注了返回的引用值，所以返回的引用值就能保证在 x 和 y 中较短的那个生命周期结束之前保持有效。

// 结构体中定义生命周期注解
struct ImportantExcerpt<'a> {
    part: &'a str,
}


// 方法定义中的生命周期注解
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}


impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

//静态生命周期
// let s: &'static str = "I have a static lifetime.";
// 'static，其生命周期能够存活于整个程序期间。


// 结合泛型类型参数、trait bounds 和生命周期
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}