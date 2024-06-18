use utils::inputln;

fn main() {
    match inputln!(<char>! "Digite um caractere: ").to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => println!("É uma vogal"),
        _ => println!("Não é uma vogal")
    }
}
