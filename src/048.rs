use utils::inputln;

fn main() {
    let mut biggest = f64::NEG_INFINITY;
    loop {
        let Ok(input) = inputln!(<f64> "Digite numero (qualquer outra coisa para cancelar): ")
        else {
            break;
        };
        if biggest < input {
            biggest = input;
        }
    }
    println!("Maior: {}", biggest)
}
