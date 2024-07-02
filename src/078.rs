fn main() {
    let (even, odd) = (1..=100).fold(
        (0, 0),
        |(p, i), x| if x % 2 == 0 { (p + 1, i) } else { (p, i + 1) },
    );
    println!("Numero de pares: {even}");
    println!("Numero de impares: {odd}");
}
