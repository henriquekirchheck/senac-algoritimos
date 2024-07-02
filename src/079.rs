fn main() {
    (5..=100).step_by(5).filter(|x| x % 10 != 0).for_each(|x| println!("{x}"))
}
