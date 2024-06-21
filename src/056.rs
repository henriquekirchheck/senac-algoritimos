use utils::inputln;

fn main() {
    println!("Media: {}", (0..10).map(|_| inputln!(<f64>! "Digite numero: ")).sum::<f64>() / 10f64);
}
