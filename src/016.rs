use utils::inputln;

fn main() {
    let num1 = inputln!(<i32>! "Digite numero 1: ");
    let num2 = inputln!(<i32>! "Digite numero 2: ");
    let num3 = inputln!(<i32>! "Digite numero 3: ");
    println!("Primeiro passo: {} * {} = {}", num1, num2, num1 * num2);
    println!("Segundo passo: {} * {} = {}", num1 * num2, num3, num1 * num2 * num3);
    println!("Final: {}", num1 * num2 * num3);
}
