pub(crate) fn mod2_print() {
    for c in ('A'..='Z').chain('a'..='z') {
        print!("{} ", c);
    }
}
