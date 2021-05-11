use std::collections::HashMap;

fn main() {
    // 新建一个哈希 map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 使用 collect 方法将这个元组 vector 转换成一个 HashMap
    let teams = vec![
        String::from("Blue"),
        String::from("Yellow"),
        String::from("Red"),
    ];
    let initial_scores = vec![10, 50, 35, 65];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("scores is {:#?}", scores);

    // 哈希 map 和所有权
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效，
    // 尝试使用它们看看会出现什么编译错误！
    // println!("{}",field_name); //borrow of moved value: `field_name`

    // 访问哈希 map 中的值
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(x) => {
            println!("the x is {}", x);
        }
        None => {
            println!("the x is None");
        }
    }

    if let Some(x) = score {
        println!("the x is {}", x);
    }

    println!("the score is {:?}", score);

    //遍历

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 更新哈希 map
    // 覆盖一个值
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // 只在键没有对应值时插入

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // 根据旧值更新一个值
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
