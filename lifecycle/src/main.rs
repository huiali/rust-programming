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

// 函数签名中的生命周期注解

