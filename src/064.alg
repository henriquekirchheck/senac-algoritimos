algoritmo "??"
var
    numero: inteiro
    i: inteiro
    counter: inteiro
    primo: logico
inicio
    numero <- 2
    enquanto (numero <= 100) faca
        primo <- verdadeiro

        i <- 2
        enquanto i <= (numero \ 2) faca
            se ((numero mod i) = 0) entao
                primo <- falso
            fimse
            i <- i + 1
        fimenquanto
        
        se (primo) entao
             escreval(numero)
        fimse

        numero <- numero + 1
    fimenquanto

fimalgoritmo