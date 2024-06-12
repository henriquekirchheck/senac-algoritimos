use utils::inputln;

fn main() {
    let idade = inputln!(<i32>! "Qual sua idade: ");
    println!("É maior de idade: {}", idade >= 18);
}