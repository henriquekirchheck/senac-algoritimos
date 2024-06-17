use utils::inputln;

fn main() {
    let numero = inputln!(<i32>! "Digite seu numero: ");
    let multiplo = inputln!(<i32>! "Digite seu multiplo: ");
    if numero % multiplo == 0 {
        println!("É múltiplo");
    } else {
        println!("Não é múltiplo")
    }
}
