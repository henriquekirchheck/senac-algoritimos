use utils::inputln;

fn main() {
    let n1 = inputln!(<i32>! "Primeiro numero: ");
    let n2 = inputln!(<i32>! "Segundo numero: ");
    let n3 = inputln!(<i32>! "Terceiro numero: ");
    println!("O resultado é: {}", n1 / n2 / n3)
}
