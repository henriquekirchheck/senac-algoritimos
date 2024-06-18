use utils::inputln;

fn main() -> Result<(), ()> {
    match inputln!(<u8>! "Digite sua idade: ") {
        ..=17 => println!("Desempregado, pois é menor de idade"),
        _ => println!(
            "Maior de idade e {}",
            match inputln!("Digita se você está empregado (true/false): ").as_str() {
                "s" | "S" | "" => "empregado",
                "n" | "N" => "desempregado",
                _ => {
                    println!("Opção Invalida");
                    return Err(());
                }
            }
        ),
    }
    Ok(())
}
