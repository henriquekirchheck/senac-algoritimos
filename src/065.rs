use utils::inputln;

fn main() {
    let mut soma = 0.;
    let mut counter = 0usize;
    while let Ok(nota) = inputln!(<f64> "Digite nota (nada para cancelar): ") {
        soma += nota;
        counter += 1;
    }
    println!("A media das notas: {}", soma / counter as f64)
}
