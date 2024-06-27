use utils::inputln;

fn main() {
    println!(
        "A soma: {}",
        (0..3)
            .map(|_| inputln!(<f64>! "Digite numero: "))
            .sum::<f64>()
    )
}
