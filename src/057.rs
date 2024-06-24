use std::io::Write;
use utils::inputln;

fn main() {
    let input = inputln!(<u32>! "Digite um numero: ");
    let mut stdlock = std::io::stdout().lock();
    (1..=input).for_each(|x| write!(stdlock, "{x} ").unwrap());
    stdlock.flush().unwrap();
}
