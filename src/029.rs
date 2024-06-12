use utils::inputln;

fn main() {
    let n1 = inputln!(<i32>! "Numero 1: ");
    let n2 = inputln!(<i32>! "Numero 2: ");
    println!("{}", if n1 == n2 { "Igual" } else { "Diferente" })
}
