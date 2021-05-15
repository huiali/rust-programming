fn main() {
    let v = vec![1, 2, 3];

    v[99];  // 缓冲区溢出

    // panic!("crash and burn");

    // backtrace 是一个执行到目前位置所有被调用的函数的列表。
    // cmd $env:RUST_BACKTRACE=1

    use std::net::IpAddr;
    // 当我们比编译器知道更多的情况
    // 当你有一些其他的逻辑来确保 Result 会是 Ok 值时，调用 unwrap 也是合适的
    // 如果通过人工检查代码来确保永远也不会出现 Err 值，那么调用 unwrap 也是完全可以接受的
    let home: IpAddr = "127.0.0.1".parse().unwrap();

    // Rust 的错误处理功能被设计为帮助你编写更加健壮的代码。
    // panic! 宏代表一个程序无法处理的状态，并停止执行而不是使用无效或不正确的值继续处理。
    // Rust 类型系统的 Result 枚举代表操作可能会在一种可以恢复的情况下失败。
    // 可以使用 Result 来告诉代码调用者他需要处理潜在的成功或失败。
    // 在适当的场景使用 panic! 和 Result 将会使你的代码在面对不可避免的错误时显得更加可靠。
}
