use utils::inputln;

fn main() {
    let mut minimum = f64::INFINITY;
    loop {
        let Ok(input) = inputln!(<f64> "Digite numero (qualquer outra coisa para cancelar): ")
        else {
            break;
        };
        if minimum > input {
            minimum = input;
        }
    }
    println!("Menor: {}", minimum)
}
