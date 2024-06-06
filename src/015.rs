use utils::inputln;

fn main() {
    let num = inputln!(<i32>! "Salario: ");
    println!("Sucessor: {}", num + 1);
    println!("Antecessor: {}", num - 1);
}
