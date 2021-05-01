fn main() {
    //变量
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // cannot assign twice to immutable variable `x` // 当变量不可变时，一旦值被绑定一个名称上，你就不能改变这个值。
    // println!("The value of x is: {}", x);

    //可变变量
    let mut x = 5; //通过 mut，允许把绑定到 x 的值从 5 改成 6
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //常量
    const MAX_POINTS: u32 = 100_000; //不允许对常量使用 mut。常量不光默认不能变，它总是不能变。
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    //隐藏（Shadowing）
    let x = 5;
    let x = x + 1; //可以定义一个与之前变量同名的新变量，而新变量会 隐藏 之前的变量
    let x = x * 2;
    let x = "123";
    println!("The value of x is: {}", x);

    //隐藏 mut
    let spaces = "   ";
    let spaces = spaces.len();

    let mut spaces = "   ";
    // spaces = spaces.len(); 不能改变变量的类型
    println!("The value of spaces is: {}", spaces);

    //数据类型
}
