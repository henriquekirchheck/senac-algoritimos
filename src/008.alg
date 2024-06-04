algoritmo "08"
var
    num: vetor [0..2] de real
    max: real
    soma: real
    i: inteiro
inicio
    max <- 3
    i <- 0
    enquanto (i<max) faca
        escreva("Digite o", i + 1, "° numero: ")
        leia(num[i])
        i <- i + 1
    fimenquanto
    i <- 0
    enquanto (i<max) faca
        soma <- soma + num[i]
        i <- i + 1
    fimenquanto
    escreval("A media:", soma / max)

fimalgoritmo