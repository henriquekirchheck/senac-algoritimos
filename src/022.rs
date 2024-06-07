use utils::inputln;

fn main() {
    let cigarros = inputln!(<u32>! "Cigarros por dia: ");
    let anos = inputln!(<f64>! "Anos fumando: ");
    println!(
        "você perdeu {} dias de vida",
        anos * 365.25 * cigarros as f64 * 10. / 60. / 24.
    )
}
