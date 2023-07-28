/**
 * rust 宏分类
 * 声明式宏 macro_rules!
 * 派生宏 proc_macro_derive
 * 属性宏 proc_macro_attribute
 *
 * 声明式宏
 * 声明式宏允许开发者以类似于函数的形式定义宏，并通过参数匹配和模板替换的方式来生成代码。
 *
 */
// 基本语法如下
macro_rules! macro_name {
    // 匹配模式和模板替换
    (pattern1) => {
        /* code1 */
    };
    (pattern2) => {
        /* code2 */
    }; // 更多模式和模板替换
}

// 声明hashmap宏
#[macro_export] // 表示该宏导出可供其他包使用
macro_rules! hashmap {
    //下面这一段是匹配参数，格式表明对参数匹配时可以对xxx部分进行0次或若干次匹配(这里*的作用类似于正则表示式的 , 当然也可以换成 ? "0或1次 或者 + "至少1次" ) ，
    //xxx参数用逗号进行分隔且最后一次匹配不能有逗号；$key: expr => $val: expr 是参数的格式，expr代表匹配的类型，可以是任何有效的rust表达式；
    ($ ($key: expr => $val: expr),*  ) => {
        {
            let mut map = std::collections::HashMap::new();
            //进行代码的执行，执行次数与上面参数匹配的次数相同，$key $val就是每次匹配到参数的值
            $(  map.insert($key, $val);  )*
             map
        }
    };
}
/// 测试代码
#[test]
fn test_hashmap() {
    let map = hashmap!(1 => "one", 2=> "two", 3=> "three" );
    println!("map {:?} ", map);
}

fn main() {
    println!("Hello, world!");
}
