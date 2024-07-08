use inquire::Select;

enum Answer {
    Corret,
    Incorrect,
}

fn main() {
    let answers = [
        question("Qual é o nome do movimento no xadrez onde o rei se move duas casas na direção da torre?", ["Roque", "En passant", "El vaticano"], 0),
        question("Quantas casas um peão se move inicialmente em sua primeira jogada?", ["1", "2", "3"], 1),
        question("Qual dessas peças pode pular outras peças no tabuleiro?", ["Bispo", "Cavalo", "Torre"],1),
        question("Qual é as duas peças que podem mover de forma diagonal no tabuleiro?", ["Bispo e Torre", "Cavalo e Torre", "Bispo e Rainha"], 2),
        question("Qual peça pode fazer um movimento chamado roque?", ["Rei", "Rainha", "Peão"], 0),
        question("Qual é o movimento especial onde um peão pode capturar uma peça inimiga que tenha avançado duas casas?", ["En passant", "Roque", "Promoção"], 0),
        question("Quantas colunas e fileiras tem um tabuleiro de xadrez padrão?", ["6 colunas, 6 fileiras", "8 colunas, 8 fileiras", "10 colunas, 10 fileiras"], 1),
        question("Qual é o objetivo principal do jogo de xadrez?", ["Capturar todas as peças adversárias", "Chegar no outro lado do tabuleiro com o rei", "Dar xeque-mate no rei adversário"], 2),
        question("Quem tem a vantagem inicial no jogo de xadrez?", ["As peças pretas", "As peças brancas", "Nenhuma das opções acima"], 1),
        question("Qual é o numero total de bispos no inicio de um jogo de xadrez", ["4", "2", "3"], 0)
    ];
    let points = answers
        .iter()
        .map(|x| match x {
            Answer::Corret => 1,
            Answer::Incorrect => 0,
        })
        .sum::<i32>();

    println!("Pontos: {points}")
}

fn question<const T: usize>(question: &str, answers: [&str; T], answer_index: usize) -> Answer {
    match Select::new(question, answers.to_vec()).prompt().unwrap() == answers[answer_index] {
        true => Answer::Corret,
        false => Answer::Incorrect,
    }
}
