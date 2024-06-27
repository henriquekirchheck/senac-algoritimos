fn main() {
    (1..=10).map(|x|(x, x * 7)).for_each(|(i, x)| println!("{i} * 7 = {x}"))
}
