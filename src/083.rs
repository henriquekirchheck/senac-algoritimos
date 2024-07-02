use utils::inputln;

fn main() {
    let mut sum = 0.0;
    let mut num = 0;
    let mut max = f64::NEG_INFINITY;
    let mut min = f64::INFINITY;
    while let Ok(x) = inputln!(<f64> "Digite nota (nada para cancelar): ") {
        sum += x;
        max = if max < x { x } else { max };
        min = if min > x { x } else { min };
        num += 1;
    }

    println!("Media: {}", sum / num as f64);
    println!("Menor: {}", min);
    println!("Maior: {}", max);
}
