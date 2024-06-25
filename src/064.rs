fn main() {
    (2..=100)
        .filter(|num| !(2..(num / 2)).any(|i| num % i == 0))
        .for_each(|x| print!("{x} "));
}
