use crate::my_macro;

#[my_macro(bar)]
fn test1() {}

fn main() {
    test1();
}
