use utils::inputln;

fn main() {
    let numero = inputln!(<i32>! "Digita numero: ");
    println!("{}", if numero & 1 == 0 { "Par" } else { "Impar" })
}
