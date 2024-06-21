algoritmo "??"
var
    input: real
    biggest: real
    counter: inteiro
    num: inteiro
inicio
    escreva("Digita numero de membros do maximo: ")
    leia(num)
    counter <- 0
    biggest <- -99999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999
    enquanto (num <> counter) faca
        escreva("Digita numero: ")
        leia(input)
        se (biggest > input) entao
            biggest <- input
        fimse
        counter <- counter + 1
    fimenquanto
    escreval("Maior:", biggest)
fimalgoritmo