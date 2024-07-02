algoritmo "??"
var
    i: inteiro
    numero: inteiro
inicio
    i <- 1
    repita
        se (i mod 2 = 0) entao
            numero <- numero + 1
        fimse
        i <- i + 1
    ate i = 21
    escreval("Pares:", numero)
fimalgoritmo