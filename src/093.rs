use utils::inputln;

fn main() {
    let nome = inputln!("Digite o nome do aluno: ");
    while let Ok(nota) = inputln!(<f64> "Digite uma nota (invalido para cancelar): ") {
        println!(
            "{nome} sua nota foi {nota} e por esse motivo você está {}",
            if nota < 60. { "reprovado" } else { "aprovado" }
        )
    }
}
