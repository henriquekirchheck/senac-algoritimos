use utils::inputln;

fn main() {
    let num = inputln!(<u128>! "Digite numero: ");
    println!("Fatorial: {}", (2..=num).product::<u128>())
}
