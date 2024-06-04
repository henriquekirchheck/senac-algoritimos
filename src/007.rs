use core::array;
use utils::inputln;

const NUM: usize = 5;
fn main() {
    let input: [i32; NUM] = array::from_fn(|n| inputln!(<i32>! "Escreva o {}° numero: ", n + 1));
    println!("A soma: {}", input.into_iter().sum::<i32>())
}
