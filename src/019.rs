use utils::inputln;

fn main() {
    let num = inputln!(<f64>! "Digite o preço: ");
    println!("Preço com desconto: {}", num * 0.95);
}
