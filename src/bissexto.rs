use utils::inputln;
fn main() {
    let ano = inputln!(<u16>! "Digite ano: ");
    let ano_bissexto = if (ano % 4 == 0 && ano % 100 != 0) || ano % 400 == 0 {
        "Bissexto"
    } else {
        "Normal"
    };
    println!("{ano_bissexto}")
}
