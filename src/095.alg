algoritmo "??"
var
    pao: inteiro
    broa: inteiro

    ganhos: real
inicio
    escreva("Digita numero de pães franceses: ")
    leia(pao)
    escreva("Digita numero de broas: ")
    leia(broa)

    ganhos <- pao * 0.15 + broa * 1.5

    escreval("Total de ganhos foi:", ganhos)
    escreval("Total para ser poupado:", ganhos / 10)
fimalgoritmo