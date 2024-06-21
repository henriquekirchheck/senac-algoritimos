use utils::inputln;

fn main() {
    let mut sum = 0f64;
    let mut counter = 0;
    loop {
        let Ok(input) = inputln!(<f64> "Digite numero (qualquer outra coisa para cancelar): ")
        else {
            break;
        };
        sum += input;
        counter += 1;
    }
    println!("Media: {}", sum / counter as f64)
}
