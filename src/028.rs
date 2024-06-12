use utils::inputln;

fn main() {
    let nota = inputln!(<f64>! "Digite a nota: ");
    println!("{}", match nota >= 7. {
        true => "Aprovado",
        false => "Reprovado",
    })
}
