use utils::inputln;

fn main() {
    match inputln!(<i32>! "Digite um numero: ") {
        ..=-1 | 101.. => println!("Negativo ou maior de 100"),
        0..=100 => println!("Entre 0 e 100"),
    }
}
