use utils::inputln;

fn main() {
    let mut sum = 0.;
    while let Some(x) = match inputln!(<f64>! "Digite numero: ") {
        0. => None,
        x if x < 0. => Some(0.),
        x => Some(x),
    } {
        sum += x
    }
    println!("Soma: {sum}")
}
