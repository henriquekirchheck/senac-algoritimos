fn main() {
    (1..=20).map(|i| i * i).for_each(|n| println!("{n}"))
}
