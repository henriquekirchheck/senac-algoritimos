use core::array;
use utils::inputln;

const NUM: usize = 3;
fn main() {
    let input: [_; NUM] = array::from_fn(|n| inputln!(<f64>! "Escreva o {}° numero: ", n + 1));
    println!("A media: {}", input.into_iter().sum::<f64>() / NUM as f64)
}
