use utils::inputln;

fn main() {
    let num = inputln!(<i32>! "Digite numero: ");
    println!(
        "{}",
        if num.is_positive() {
            "Positivo"
        } else {
            "Negativo"
        }
    )
}
