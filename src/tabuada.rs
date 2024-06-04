use utils::inputln;

fn main() {
    let input = inputln!("Escolha um numero: ");
    let trimmed_input = input.trim();
    let number = trimmed_input.parse::<usize>().unwrap();
    for n in 1..=10 {
        println!(
            "{number} * {n:2} = {:n_size$}",
            number * n,
            n_size = trimmed_input.len() + 1
        )
    }
}
