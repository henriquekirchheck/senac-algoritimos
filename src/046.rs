use utils::inputln;

fn main() {
    let num = inputln!(<u64>! "Digite numero: ");
    println!("Numero Primo: {}", !(2..(num / 2)).any(|i| num % i == 0));
}
