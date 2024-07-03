use utils::inputln;

fn main() {
    println!(
        "Media: {}",
        (0..5)
            .map(|_| inputln!(<i32>! "Digite numero: "))
            .sum::<i32>()
            / 5
    )
}
