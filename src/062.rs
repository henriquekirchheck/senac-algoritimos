use utils::inputln;

fn main() {
    (1..=inputln!(<u32>! "Digite um numero"))
        .filter(|x| x % 5 != 0)
        .for_each(|x| print!("{x} "));
}
