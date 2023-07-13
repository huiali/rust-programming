`自己对所有权、不可变引用、可变引用这三者的规则或特性做一个集中的总结`


Rust的所有权(ownership)机制规定：Rust中的每个值都有一个被称为其所有者（owner）的变量，并且有且只能有唯一的所有者。
Rust中的引用(references)允许使用值但不获取其所有权，这种操作也被称为所有权借用（borrowing）。通过＆T和＆mut T将引用分为两种：

不可变引用（&T），也被称为共享引用，所有者可以读取引用指向的数据，但不能修改数据。
可变引用（&mut T）也被称为独占引用，不能有别名，在同一时刻，同一个值不可能存在别的引用。

```
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s; // ERROR: 可变引用，不能有别名
    println!("{}, {}", r1, r2);
    let x = 1;
    let y = &mut x; // ERROR: 当有一个不可变值时，不能可变的借用它
}
```
Rust内存安全性基于以下规则：给定对象T，则只能具有以下之一：

对象有几个不可变的引用（&T），也称为别名（aliasing）。
对象有一个可变引用（&mut T），也称为可变性（mutability）。
这由Rust编译器强制执行。但是，在某些情况下，此规则不够灵活。有时需要对一个对象有多个引用并对其进行改变。
```
fn main() {
			
    let mut data = 1_i32;
    let p : &i32 = &data;	// 两个变量data和p访问同一块内存区域，称为别名
    data = 10;				// ERROR: 存在别名时，不能同时提供可变性
    println!("{}", *p);
}
```
在Rust中，一个变量是否是可变的，取决于是否用mut修饰变量绑定。如果我们用let var : T声明，那么var是不可变的；而且，var内部所有的成员也都是不可变的；如果我们用let mut var : T声明，那么var是可变的，相应的它的内部所有成员也都是可变的。



* 1、所有权
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

总结：
// 1、变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。
// 2、当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。
// 3、在每一个函数中都获取所有权并接着返回所有权有些啰嗦。
// 4、如果我们想要函数使用一个值但不获取所有权该怎么办呢？
// 5、果我们还要接着使用它的话，每次都传进去再返回来就有点烦人了，
// 6、除此之外，我们也可能想返回函数体中产生的一些数据。


* 2、可变引用
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


* 3、 不可变引用

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
