algoritmo "??"
var
    i: inteiro
    numero: inteiro
    primo: logico
inicio  
    escreva("Digita numero: ")
    leia(numero)
    primo <- verdadeiro
    para i de 2 ate numero \ 2 faca
        se (numero mod i = 0) entao
            primo <- falso
        fimse
    fimpara
    escreval("Primo: ", primo)
fimalgoritmo