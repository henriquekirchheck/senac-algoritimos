use utils::inputln;

fn main() {
    match inputln!(<i32>! "Digite numero: ") {
        x if x % 2 == 1 || x % 5 == 0 => println!("É número ímpar ou múltiplo de 5"),
        _ => println!("Não é número ímpar ou múltiplo de 5")
    }
}
