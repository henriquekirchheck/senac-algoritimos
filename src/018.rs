use utils::inputln;

fn main() {
    let num = inputln!(<f64>! "Digite salario: ");
    println!("Salario com aumento: {}", num * 1.15);
}
