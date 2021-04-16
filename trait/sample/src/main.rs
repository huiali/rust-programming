use core::fmt::Debug;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    notify(tweet);

    //println!("1 newe tweet:{}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    println!("New article available!{}", article.summarize());
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    // fn summarize(&self) -> String;

    //默认实现
    fn summarize(&self) -> String {
        format!("(Read more from {} ....)", self.summarize_author())
    }
}

pub trait Display {}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {}({})", self.headline, self.author, self.location)
    // }
    fn summarize_author(&self) -> std::string::String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}:{}", self.username, self.content)
    }
    fn summarize_author(&self) -> std::string::String {
        todo!()
    }
}

//使用trait作为参数
pub fn notify(item: impl Summary) {
    println!("Notify-->{}", item.summarize());
}

//trait 约束
pub fn notify2<T: Summary>(item: T) {
    println!("Notify-->{}", item.summarize());
}

//通过 + 语法来指定多个trait约束
pub fn notify3(item: impl Summary + Display) {
    println!("Notify-->{}", item.summarize());
}
pub fn notify4<T: Summary + Display>(item: T) {
    println!("Notify-->{}", item.summarize());
}

//使用where 从句简化trait约束
fn some_function<T: Summary + Copy, U: Clone + Debug>(t: T, u: U) -> i32 {
    1
}

fn some_function2<T, U>(t: T, u: U) -> i32
where
    T: Summary + Copy,
    U: Clone + Debug,
{
    2
}
