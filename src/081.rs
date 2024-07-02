use utils::{inputln, is_prime};

fn main() {
    println!(
        "{} primo",
        match is_prime(inputln!(<usize>! "Digite um numero: ")) {
            true => "É",
            false => "Não é",
        }
    )
}
