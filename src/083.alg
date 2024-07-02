algoritmo "??"
var
    i: inteiro
    numero_de_notas: inteiro
    nota: real
    soma: real
    max: real
    min: real
inicio
    escreva("Digita numero de notas: ")
    leia(numero_de_notas)

    min <- 9999999999999999999999999
    max <- -9999999999999999999999999
    soma <- 0

    para i de 1 ate numero_de_notas faca
        escreva ("Digite a nota numero", i, ": ")
        leia (nota)
        soma <- soma + nota
        se (nota < min) entao
            min <- nota
        fimse
        se (nota > max) entao
            max <- nota
        fimse
    fimpara
    escreval("Media:", soma / numero_de_notas)
    escreval("Menor:", min)
    escreval("Maior:", max)
fimalgoritmo