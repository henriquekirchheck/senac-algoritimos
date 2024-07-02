fn main() {
    (1..=30)
        .map(|i| (i, i % 2 == 0))
        .for_each(|(x, i)| println!("{}: {}", if i { "Par" } else { "Impar" }, x))
}
