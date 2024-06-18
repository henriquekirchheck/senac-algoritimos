algoritmo "??"
var
    idade: inteiro
    empregado: caractere
inicio
    escreva("Digita idade: ")
    leia(idade)

    se (idade >= 18) entao
        escreva("Digita se você está empregado [S/n]: ")
        leia(empregado)
        escolha empregado
            caso "S", "s", ""
                escreval ("Maior de idade e empregado")
            caso "N", "n"
                escreval ("Maior de idade e desempregado")
            outrocaso
                escreval ("Opção Invalida")
        fimescolha
    senao
        escreval ("Desempregado, pois é menor de idade")
    fimse

fimalgoritmo