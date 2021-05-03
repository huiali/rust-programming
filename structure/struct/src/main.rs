// struct，或者 structure，是一个自定义数据类型，
// 允许你命名和包装多个相关的值，从而形成一个有意义的组合。

fn main() {
    let mut user1 = User {
        sign_in_count: 1,
        active: true,
        username: String::from("zhangsan"),
        email: String::from("zhangsan@hotmail.com"),
    };

    user1.email = String::from("anotheremail@example.com");

    // 使用结构体更新语法从其他实例创建实例
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// * 定义并实例化结构体

///User 结构体定义 key: value
struct User {
    username: String, //字段（field）
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 变量与字段同名时的字段初始化简写语法

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// * 元组结构体（tuple structs）

// 没有任何字段的类单元结构体
// 使用没有命名字段的元组结构体来创建不同的类型

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
