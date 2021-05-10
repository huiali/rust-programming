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


    
}
