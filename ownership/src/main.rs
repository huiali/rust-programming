// 所有权（系统）是 Rust 最为与众不同的特性，它让 Rust 无需垃圾回收（garbage collector）即可保障内存安全。
// 因此，理解 Rust 中所有权如何工作是十分重要的。
// 借用、slice 以及 Rust 如何在内存中布局数据。

// * 所有权规则

// Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
// 值在任一时刻有且只有一个所有者。
// 当所有者（变量）离开作用域，这个值将被丢弃。

fn main() {
    // * 变量作用域
    // 作用域是一个项（item）在程序中有效的范围
    let q = "hello";

    {
        // s 在这里无效, 它尚未声明
        let s = "hello"; // 从此处起，s 是有效的
                         // 使用 s
    }
    // 当 s 进入作用域 时，它就是有效的。
    // 这一直持续到它 离开作用域 为止。

    // * String 类型

    // 我们已经见过字符串字面值，即被硬编码进程序里的字符串值。
    // 字符串字面值是很方便的，不过它们并不适合使用文本的每一种场景。
    // 原因之一就是它们是不可变的。
    // 另一个原因是并非所有字符串的值都能在编写代码时就知道：例如，要是想获取用户输入并存储该怎么办呢？
    // 为此，Rust 有第二个字符串类型，String。
    // 这个类型被分配到堆上，所以能够存储在编译时未知大小的文本。
    // 可以使用 from 函数基于字符串字面值来创建 String，如下：
    let s = String::from("hello");
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("{}", s); // 将打印 `hello, world!`

    // * 内存与分配

    // 在有 垃圾回收（garbage collector，GC）的语言中， GC 记录并清除不再使用的内存，而我们并不需要关心它。
    // 没有 GC 的话，识别出不再使用的内存并调用代码显式释放就是我们的责任了，跟请求内存的时候一样。
    // 从历史的角度上说正确处理内存回收曾经是一个困难的编程问题。
    // 如果忘记回收了会浪费内存。如果过早回收了，将会出现无效变量。
    // 如果重复回收，这也是个 bug。我们需要精确的为一个 allocate 配对一个 free。

    // 对于GC ，Rust 采取了一个不同的策略：内存在拥有它的变量离开作用域后就被自动释放。

    {
        let s = String::from("hello"); // 从此处起，s 是有效的

        // 使用 s
    } // 此作用域已结束，
      // s 不再有效

    //   这是一个将 String 需要的内存返回给操作系统的很自然的位置：当 s 离开作用域的时候。
    //   当变量离开作用域，Rust 为我们调用一个特殊的函数。
    //   这个函数叫做 drop，在这里 String 的作者可以放置释放内存的代码。
    //   Rust 在结尾的 } 处自动调用 drop。

    // * 变量与数据交互的方式（一）：移动

    let x = 5;
    let y = x;
    // 这两个 5 被放入了栈中

    let s1 = String::from("hello");
    let s2 = s1;

    // 将 s1 赋值给 s2，String 的数据被复制了，
    // 这意味着我们从栈上拷贝了它的指针、长度和容量。我们并没有复制指针指向的堆上数据。

    // 当变量离开作用域后，Rust 自动调用 drop 函数并清理变量的堆内存。
    // 两个数据指针指向了同一位置。这就有了一个问题：当 s2 和 s1 离开作用域，他们都会尝试释放相同的内存。
    // 这是一个叫做 二次释放（double free）的错误，也是之前提到过的内存安全性 bug 之一。
    // 两次释放（相同）内存会导致内存污染，它可能会导致潜在的安全漏洞。

    // Rust 里面不会做类似于浅拷贝指针、长度和容量，而是移动指针所有权。

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1);   // value borrowed here after move

    // * 变量与数据交互的方式（二）：克隆

    // 如果确实 需要深度复制 String 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 clone 的通用函数。
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // * 只在栈上的数据：拷贝

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
    // Rust 有一个叫做 Copy trait 的特殊注解，可以用在类似整型这样的存储在栈上的类型上
    // 任何简单标量值的组合可以是 Copy 的，不需要分配内存或某种形式资源的类型是 Copy 的

    // 所有整数类型，比如 u32。
    // 布尔类型，bool，它的值是 true 和 false。
    // 所有浮点数类型，比如 f64。
    // 字符类型，char。
    // 元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。

    // * 所有权与函数

    // 向函数传递值可能会移动或者复制

    let s = String::from("hello"); // s 进入作用域
    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效
    let x = 5; // x 进入作用域
    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，所以在后面可继续使用 x
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

// * 返回值与作用域
// 返回值也可以转移所有权。

fn main1() {
    let s1 = gives_ownership1(); // gives_ownership 将返回值移给 s1
    let s2 = String::from("hello"); // s2 进入作用域
    let s3 = takes_and_gives_back1(s2); // s2 被移动到takes_and_gives_back 中,它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership1() -> String {
    // gives_ownership 将返回值移动给调用它的函数
    let some_string = String::from("hello"); // some_string 进入作用域.
    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back1(a_string: String) -> String {
    // a_string 进入作用域
    a_string // 返回 a_string 并移出给调用的函数
}

// 变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。
// 当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。
// 在每一个函数中都获取所有权并接着返回所有权有些啰嗦。
// 如果我们想要函数使用一个值但不获取所有权该怎么办呢？
// 如果我们还要接着使用它的话，每次都传进去再返回来就有点烦人了，
// 除此之外，我们也可能想返回函数体中产生的一些数据。

fn main2() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度
    (s, length)
}

// Rust 对此提供了一个功能，叫做 引用（references）,参见 引用。
