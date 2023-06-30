mod mod2;

fn main() {
    for c in ('a'..='z').chain('A'..='Z') {
        print!("{} ", c);
    }

    println!("{} ", "");
    mod2::mod2_print();
}
