use utils::inputln;

fn main() {
    let n1 = inputln!(<i32>! "O primeiro numero: ");
    let n2 = inputln!(<i32>! "O segundo numero: ");
    match (n1 & 1 == 0, n2 & 1 == 0) {
        (true, true) => println!("Ambos são par"),
        (true, false) => println!("O primeiro é par"),
        (false, true) => println!("O segundo é par"),
        (false, false) => println!("Ambos são impar"),
    }
    println!("Resto da divisão inteira por 2 do primeiro: {}", n1 % 2);
    println!("Resto da divisão inteira por 2 do segundo: {}", n2 % 2);
}
