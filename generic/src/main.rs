fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // Point<T> 的定义中只使用了一个泛型类型，这个定义表明结构体 Point<T> 对于一些类型 T 是泛型的，而且字段 x 和 y 都是 相同类型的
    // let wont_work = Point { x: 5, y: 4.0 };

    //多泛型类型
    let integer_and_float = Point1 { x: 5, y: 4.0 };
}

// 在函数中定义泛型
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// 在结构体中定义泛型
struct Point<T> {
    x: T,
    y: T,
}

struct Point1<T, U> {
    x: T,
    y: U,
}

//方法中定义泛型，在 impl 之后声明泛型 T ，这样 Rust 就知道 Point 的尖括号中的类型是泛型而不是具体类型
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 可以选择为 Point<f32> 实例实现方法，而不是为泛型 Point 实例
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 结构体定义中的泛型类型参数并不总是与结构体方法签名中使用的泛型是同一类型
impl<T, U> Point1<T, U> {
    fn mixup<V, W>(self, other: Point1<V, W>) -> Point1<T, W> {
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}
// let p1 = Point { x: 5, y: 10.4 };
// let p2 = Point { x: "Hello", y: 'c'};
// let p3 = p1.mixup(p2);

// 枚举定义中的泛型
enum Result<T, E> {
    Ok(T),
    Err(E),
}


// 泛型代码的性能
// Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。
// 单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。
fn f() {
    let integer = Some(5);
    let float = Some(5.0);
}
// Rust 编译这些代码的时候，它会进行单态化。编译器会读取传递给 Option<T> 的值并发现有两种 Option<T>：一个对应 i32 另一个对应 f64。
// 为此，它会将泛型定义 Option<T> 展开为 Option_i32 和 Option_f64，接着将泛型定义替换为这两个具体的定义。
// enum Option_i32 {
//     Some(i32),
//     None,
// }

// enum Option_f64 {
//     Some(f64),
//     None,
// }

// fn main() {
//     let integer = Option_i32::Some(5);
//     let float = Option_f64::Some(5.0);
// }