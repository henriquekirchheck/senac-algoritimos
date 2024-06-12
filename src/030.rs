use utils::inputln;

fn main() {
    let preco = inputln!(<f64>! "Preço: ");
    println!("O preço é: {}", preco * if preco > 100. { 0.9 } else { 1. })
}
