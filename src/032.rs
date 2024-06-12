use chrono::{Local, NaiveDate};
use utils::inputln;

fn main() {
    let nascimento = inputln!("Digite data de nascimento: ");
    let date = nascimento
        .split("/")
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("-")
        .parse::<NaiveDate>()
        .unwrap();
    let diff = Local::now().date_naive() - date;
    let deve_votar = match (diff.num_days() as f64 / 365.25) as u64 {
        ..=15 => "Não",
        16..=17 | 71.. => "Talvez",
        18..=70 => "Sim",
    };
    println!("Deve votar: {}", deve_votar);
}
