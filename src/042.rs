use utils::inputln;

fn main() {
    match inputln!(<u8>! "Digite sua idade: ") {
        0..=11 => println!("Criança"),
        12..=17 => println!("Adolescente"),
        18..=59 => println!("Adulto"),
        60.. => println!("Idoso"),
    }
}
