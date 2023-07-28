use core::fmt::Debug;
use std::f32::consts::PI;

fn main() {
    let triangle = Triangle {
        side1: 1.2,
        side2: 2.5,
        side3: 1.9,
    };
    notify(triangle, "三角形");

    let circular = Circular { r: 2.68 };
    notify(circular, "圆形");
}

//定义Trait
pub trait Add {
    fn sum(&self) -> f32;

    //默认实现
    fn summarize(&self) -> String {
        format!("(周长为 {})", self.sum())
    }
}

pub trait Display {}

//定义三角形
pub struct Triangle {
    pub side1: f32,
    pub side2: f32,
    pub side3: f32,
}

//计算三角形周长
impl Add for Triangle {
    fn sum(&self) -> f32 {
        &self.side1 + &self.side2 + &self.side3
    }
}

//定义圆形
pub struct Circular {
    pub r: f32,
}

//计算圆形周长
impl Add for Circular {
    fn sum(&self) -> f32 {
        2.0 * PI * self.r
    }
}

//使用trait作为参数
pub fn notify<A: Add>(item: A, name: &str) {
    println!("{}-->{}", name, item.summarize());
}

//trait 约束
pub fn notify2<T: Add>(item: T) {
    println!("Notify-->{}", item.summarize());
}

//通过 + 语法来指定多个trait约束
pub fn notify3(item: impl Add + Display) {
    println!("Notify-->{}", item.summarize());
}

//使用where 从句简化trait约束
fn some_function<T: Add + Copy, U: Clone + Debug>(t: T, u: U) -> i32 {
    3
}

fn some_function2<T, U>(t: T, u: U) -> i32
where
    T: Add + Copy,
    U: Clone + Debug,
{
    2
}
