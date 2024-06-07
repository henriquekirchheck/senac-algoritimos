use utils::inputln;

fn main() {
    let dias = inputln!(<i32>! "Quantos dias: ");
    println!("Ganhou: R${}", dias * 8 * 25);
}
