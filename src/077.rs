fn main() {
    (1..=50)
        .filter(|x| x % 2 == 0 && x % 3 == 0)
        .for_each(|x| println!("{x}"))
}
