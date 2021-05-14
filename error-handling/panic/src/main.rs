fn main() {
    let v = vec![1, 2, 3];

    v[99];  // 缓冲区溢出

    // panic!("crash and burn");

    // backtrace 是一个执行到目前位置所有被调用的函数的列表。
    // cmd $env:RUST_BACKTRACE=1
}
