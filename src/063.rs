fn main() {
    (1..=100).filter(|x| x % 3 != 0).for_each(|x| print!("{x} "));
}
