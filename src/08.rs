use core::array;
use utils::inputln;

const NUM: usize = 3;
fn main() {
    let input: [i32; NUM] =
        array::from_fn(|n| n + 1).map(|n| inputln!(<i32>! "Escreva o {}° numero: ", n));
    println!(
        "A media: {}",
        input.into_iter().sum::<i32>() as f64 / NUM as f64
    )
}
