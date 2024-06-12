use utils::inputln;

fn main() {
    let num = inputln!(<i32>! "O numero: ");
    println!("O numero é par: {}", (num & 1 == 0));
    println!("O resto da divisão por dois é: {}", num % 2);
}
