use utils::inputln;

fn main() {
    let pao = inputln!(<u32>! "Digite numero de pães:");
    let broa = inputln!(<u32>! "Digite numero de broas:");
    let total = pao as f64 * 0.15 + broa as f64 * 1.5;

    println!("Total de vendas: {total}");
    println!("Total para poupar: {}", total / 10.)
}
