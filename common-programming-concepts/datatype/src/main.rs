fn main() {
    // 在 Rust 中，每一个值都属于某一个 数据类型（data type），这告诉 Rust 它被指定为何种数据，以便明确数据处理方式。我们将看到两类数据类型子集：标量（scalar）和复合（compound）。
    // Rust 是 静态类型（statically typed）语言，也就是说在编译时就必须知道所有变量的类型。根据值及其使用方式，编译器通常可以推断出我们想要用的类型。当多种类型均有可能时，
    let guess: u32 = "55".parse().expect("Not a number!");

    // 标量类型 -----

    // 标量（scalar）类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。

    // 整型

    // 长度         有符号	 无符号
    // 8-bit	    i8  	  u8
    // 16-bit	    i16 	  u16
    // 32-bit	    i32 	  u32
    // 64-bit	    i64 	  u64
    // 128-bit	    i128	  u128
    // arch	        isize	  usize

    // 每一个有符号的变体可以储存包含从 -(2n - 1) 到 2n - 1 - 1 在内的数字，这里 n 是变体使用的位数。
    // 所以 i8 可以储存从 -(27) 到 27 - 1 在内的数字，也就是从 -128 到 127。
    // 无符号的变体可以储存从 0 到 2n - 1 的数字，所以 u8 可以储存从 0 到 28 - 1 的数字，也就是从 0 到 255。

    // isize 和 usize 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的。

    //     数字字面值	               例子
    //  Decimal (十进制)	         98_222
    //  Hex (十六进制)	             0xff
    //  Octal (八进制)	             0o77
    //  Binary (二进制)	             0b1111_0000
    //  Byte (单字节字符)(仅限于u8)	  b'A'

    // 浮点型

    // Rust 的浮点数类型是 f32 和 f64，分别占 32 位和 64 位。默认类型是 f64
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // 数值运算
    // 基本数学运算：加法、减法、乘法、除法和取余
    // 加法
    let sum = 5 + 10;
    // 减法
    let difference = 95.5 - 4.3;
    // 乘法
    let product = 4 * 30;
    // 除法
    let quotient = 56.7 / 32.2;
    // 取余
    let remainder = 43 % 5;

    // 布尔型
    let t = true;
    let f: bool = false; // 显式指定类型注解

    // 字符类型

    // Rust 的 char 类型的大小为四个字节(four bytes)，并代表了一个 Unicode 标量值（Unicode Scalar Value）
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // 复合类型 ----

    // 复合类型（Compound types）可以将多个值组合成一个类型。Rust 有两个原生的复合类型：元组（tuple）和数组（array）。

    // 元组类型
    // 元组是一个将多个其他类型的值组合进一个复合类型的主要方式。元组长度固定：一旦声明，其长度不会增大或缩小。
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    // 程序首先创建了一个元组并绑定到 tup 变量上。
    // 接着使用了 let 和一个模式将 tup 分成了三个不同的变量，x、y 和 z。这叫做 解构（destructuring），因为它将一个元组拆成了三个部分。最后，程序打印出了 y 的值

    // 除了使用模式匹配解构外，也可以使用点号（.）后跟值的索引来直接访问它们。
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // 数组类型
    // 数组是一整块分配在栈上的内存。
    // 数组中的每个元素的类型必须相同。Rust 中的数组与一些其他语言中的数组不同，因为 Rust 中的数组是固定长度的：一旦声明，它们的长度不能增长或缩小。
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
