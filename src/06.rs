use utils::inputln;

fn main() {
    let n1 = inputln!(<f64>! "Digite o 1° numero: ");
    let n2 = inputln!(<f64>! "Digite o 2° numero: ");
    let n3 = inputln!(<f64>! "Digite o 3° numero: ");

    println!("A soma: {}", n1 + n2 + n3)
}
