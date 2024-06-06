use utils::inputln;

fn main() {
    let nome = inputln!("Nome do Funcionario: ");
    let num = inputln!(<f64>! "Salario: ");
    println!("O funcionario/a {nome} tem um salario de R${num} em Julho");
}
