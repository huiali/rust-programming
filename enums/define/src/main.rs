// 枚举（enumerations），也被称作 enums。
// 枚举允许你通过列举可能的 成员（variants） 来定义一个类型。
// 首先，我们会定义并使用一个枚举来展示它是如何连同数据一起编码信息的。
// 接下来，我们会探索一个特别有用的枚举，叫做 Option，它代表一个值要么是某个值要么什么都不是。
// 然后会讲到在 match 表达式中用模式匹配，针对不同的枚举值编写相应要执行的代码。最后会介绍 if let，另一个简洁方便处理代码中枚举的结构。

// 枚举是一个很多语言都有的功能，不过不同语言中其功能各不相同。
// Rust 的枚举与 F#、OCaml 和 Haskell 这样的函数式编程语言中的 代数数据类型（algebraic data types）最为相似。

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home is {:#?}", home);
    println!("loopback is {:#?}", loopback);

    let home2 = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddr2::V6(String::from("::1"));

    println!("home2 is {:#?}", home2);
    println!("loopback2 is {:#?}", loopback2);

    let home3 = IpAddr3::V4(127, 0, 0, 1);
    let loopback3 = IpAddr3::V6(String::from("::1"));

    println!("home3 is {:#?}", home3);
    println!("loopback3 is {:#?}", loopback3);

    //调用枚举的impl 方法
    let m = Message::Write(String::from("hello"));
    m.call();
}

// V4 和 V6。这被称为枚举的 成员（variants）：
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_type: IpAddrKind) {}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
// 可以使用一种更简洁的方式来表达相同的概念，仅仅使用枚举并将数据直接放进每一个枚举成员而不是将枚举作为结构体的一部分。
// IpAddr 枚举的新定义表明了 V4 和 V6 成员都关联了 String 值：
enum IpAddr2 {
    V4(String),
    V6(String),
}
#[derive(Debug)]
// 每个成员可以处理不同类型和数量的数据。
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Quit 没有关联任何数据。
// Move 包含一个匿名结构体。
// Write 包含单独一个 String。
// ChangeColor 包含三个 i32。
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

// 结构体和枚举还有另一个相似点：就像可以使用 impl 来为结构体定义方法那样，也可以在枚举上定义方法。这是一个定义于我们 Message 枚举上的叫做 call 的方法：
// Option 枚举和其相对于空值的优势
