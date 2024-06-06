algoritmo "007"
var
    num: vetor [0..4] de inteiro
    max: inteiro
    soma: inteiro
    i: inteiro
inicio
    max <- 5
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
    escreval("A soma:", soma)

fimalgoritmo