use utils::inputln;

fn main() {
    println!(
        "Media: {}",
        (1..=4)
            .map(|_| inputln!(<f64>! "Digite numero: "))
            .sum::<f64>()
            / 4.
    )
}
