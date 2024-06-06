use utils::inputln;

fn main() {
    let _nome = inputln!("Digite nome: ");
    let num1 = inputln!(<f64>! "Digite numero 1: ");
    let num2 = inputln!(<f64>! "Digite numero 1: ");
    let num3 = inputln!(<f64>! "Digite numero 1: ");
    println!("Produto: {}", num1 * num2 * num3)
}
