use utils::inputln;

fn main() {
    println!(
        "A media: {}",
        (0..4)
            .map(|_| inputln!(<f64>! "Digite numero: "))
            .sum::<f64>()
            / 4.
    )
}
