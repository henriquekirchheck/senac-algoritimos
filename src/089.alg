algoritmo "??"
var
    numero: real
    soma: real
inicio
    repita
        escreva("Digita numero: ")
        leia(numero)
        se (numero < 0) entao
            soma <- soma + numero
        fimse
    ate numero = 0
    escreval("Soma:", soma)
fimalgoritmo