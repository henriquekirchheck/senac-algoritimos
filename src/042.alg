algoritmo "??"
var
    idade: inteiro
inicio
    escreva("Digita idade: ")
    leia(idade)

    se (idade < 0) entao
        escreval ("Idade Invalida")
    senao
        escolha idade
            caso 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11
                escreval("Criança")
            caso 12, 13, 14, 15, 16, 17
                escreval("Adolescente")
            outrocaso
                se (idade < 60) entao
                    escreval ("Adulto")
                senao
                    escreval ("Idoso")
                fimse
        fimescolha
    fimse

fimalgoritmo