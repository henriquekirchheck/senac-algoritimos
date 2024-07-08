algoritmo "??"
var
    nome_aluno: caractere
    nota: real
    n_notas: inteiro
    i: inteiro
inicio
    escreva("Digite o nome do aluno: ")
    leia(nome_aluno)
    escreva("Digite o numero de notas para serem calculadas: ")
    leia(n_notas)
    repita
        i <- i + 1
        escreva("Digite a nota numero", i, ": ")
        leia(nota)
            escreval(nome_aluno, " sua nota numero", i, " foi", nota, " e por esse motivo você está ")
        se (nota < 60) entao
            escreval ("reprovado")
        senao
            escreval ("aprovado")
        fimse
    ate i = n_notas
fimalgoritmo