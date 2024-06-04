use core::array;
use utils::inputln;

const NUM: usize = 5;
fn main() {
    let input: [i32; NUM] =
        array::from_fn(|n| n + 1).map(|n| inputln!(<i32>! "Escreva o {}° numero: ", n));
    for (i, n) in input.iter().enumerate() {
        println!("O seu {}° numero é: {n}", i + 1)
    }
}
