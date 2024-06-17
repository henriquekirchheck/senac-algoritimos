use utils::inputln;

fn main() {
    let idade = inputln!(<u8>! "Qual a sua idade? ");
    let renda = inputln!(<f64>! "Qual a sua renda? R$");

    let mut errors = vec![];
    if idade < 18 {
        errors.push("Menor de idade")
    }
    if renda < 3000.0 {
        errors.push("Renda menor de R$3.000")
    }

    if !errors.is_empty() {
        println!("Você não pode fazer o seguro devido ao seguintes problemas");
        for error in errors {
            println!("- {}", error);
        }
        return;
    }

    println!("Parabens! Pode fazer o seguro!")
}
