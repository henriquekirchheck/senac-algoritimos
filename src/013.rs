use utils::inputln;

fn main() {
    let num1 = inputln!(<f64>! "Digite nota 1: ");
    let num2 = inputln!(<f64>! "Digite nota 2: ");
    println!("Produto: {}", num1 * num2 / 2f64)
}
