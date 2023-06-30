fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
