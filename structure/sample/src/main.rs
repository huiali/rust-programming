fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );

    //使用元组
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    //使用结构体
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect2);

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect2)
    );
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

// 使用元组重构
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// 使用结构体重构
#[derive(Debug)] //在结构体定义之前加上 #[derive(Debug)] 注解,允许打印
struct Rectangle {
    width: u32,
    height: u32,
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
