use utils::inputln;

fn main() {
    let nome = inputln!("Digite seu nome: ");
    let sobrenome = inputln!("Digite seu sobrenome: ");
    let nome_completo = format!("{nome} {sobrenome}");
    println!("Seu nome completo: {nome_completo}")
}
