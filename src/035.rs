use utils::inputln;

fn main() {
    let numero = inputln!(<i32>! "Digite seu numero: ");
    if numero % 5 == 0 {
        println!("Divisível por 5");
    } else {
        println!("Não divisível por 5")
    }
}
