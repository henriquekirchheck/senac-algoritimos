use utils::inputln;

fn main() {
    let input = inputln!(<usize>! "Escolha um numero: ");
    for n in 1..=10 {
        println!(
            "{input} * {n:2} = {:n_size$}",
            input * n,
            n_size = input.to_string().len() + 1
        )
    }
}
