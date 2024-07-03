algoritmo "??"
var
    i: inteiro
    contagem: inteiro
inicio
    i <- 1
    repita
        se (i mod 3 = 0) entao
            contagem <- contagem + 1
        fimse
        i <- i + 1
    ate i = 101
    escreval("Numero de divisiveis por 3:", contagem)
fimalgoritmo