use utils::inputln;

fn main() {
    let altura = inputln!(<f64>! "Digite altura: ");
    let largura = inputln!(<f64>! "Digite largura: ");
    println!("A area é: {}m^2", altura * largura)
}
