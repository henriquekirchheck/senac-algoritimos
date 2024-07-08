procedimento pergunta(pergunta, resposta_a, resposta_b, resposta_c, correta: caractere)
var
    resposta: caractere
inicio
    escreval(pergunta)
    escreval("a. ", resposta_a)
    escreval("b. ", resposta_b)
    escreval("c. ", resposta_c)

    escreva("Digite sua resposta: ")
    leia(resposta)

    se (resposta = correta) entao
        escreval ("Correto!")
        pontos <- pontos + 1
    senao
        escreval ("ERRADO!!!")
    fimse
    escreval()
fimprocedimento

algoritmo "??"
var
    pontos: inteiro
inicio

    pergunta("Qual é o nome do movimento no xadrez onde o rei se move duas casas na direção da torre?", "Roque", "En passant", "El vaticano", "a")
    pergunta("Quantas casas um peão se move inicialmente em sua primeira jogada?", "1", "2", "3", "b")
    pergunta("Qual dessas peças pode pular outras peças no tabuleiro?", "Bispo", "Cavalo", "Torre", "b")
    pergunta("Qual é as duas peças que podem mover de forma diagonal no tabuleiro?", "Bispo e Torre", "Cavalo e Torre", "Bispo e Rainha", "c")
    pergunta("Qual peça pode fazer um movimento chamado roque?", "Rei", "Rainha", "Peão", "a")
    pergunta("Qual é o movimento especial onde um peão pode capturar uma peça inimiga que tenha avançado duas casas?", "En passant", "Roque", "Promoção", "a")
    pergunta("Quantas colunas e fileiras tem um tabuleiro de xadrez padrão?", "6 colunas, 6 fileiras", "8 colunas, 8 fileiras", "10 colunas, 10 fileiras", "a")
    pergunta("Qual é o objetivo principal do jogo de xadrez?", "Capturar todas as peças adversárias", "Chegar no outro lado do tabuleiro com o rei", "Dar xeque-mate no rei adversário", "c")
    pergunta("Quem tem a vantagem inicial no jogo de xadrez?", "As peças pretas", "As peças brancas", "Nenhuma das opções acima", "b")
    pergunta("Qual é o numero total de bispos no inicio de um jogo de xadrez", "4", "2", "3", "a")

    escreval("Pontos:", pontos)
fimalgoritmo
