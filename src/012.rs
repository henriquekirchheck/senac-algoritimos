use utils::inputln;

fn main() {
    let _nome = inputln!("Digite nome 1: ");
    let _nome = inputln!("Digite nome 2: ");
    let num1 = inputln!(<f64>! "Digite numero 1: ");
    let num2 = inputln!(<f64>! "Digite numero 2: ");
    let num3 = inputln!(<f64>! "Digite numero 3: ");
    let num4 = inputln!(<f64>! "Digite numero 4: ");
    let num5 = inputln!(<f64>! "Digite numero 5: ");
    println!("Produto: {}", num1 * num2 * num3 * num4 * num5)
}
