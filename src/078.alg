algoritmo "??"
var
    i: inteiro
    pares: inteiro
    impares: inteiro
inicio
    para i de 1 ate 100 faca
        se (i mod 2 = 0) entao
            pares <- pares + 1
        senao
            impares <- impares + 1
        fimse
    fimpara
    escreval("Numeros pares:", pares)
    escreval("Numeros impares:", impares)
fimalgoritmo