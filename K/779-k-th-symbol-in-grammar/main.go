package main

import "fmt"

func kthGrammar(N int, K int) int {

	var recur func(N int, K int, S int) int

	recur = func(N int, K int, S int) int {
		if N == 0 {
			return S + K
		}

		d := K / (1<<N)
		return recur(N-1, K%(1<<N), S+d)
	}

	return (recur(N,K-1,0)%2)
}

func main() {
	fmt.Println(kthGrammar(10, 1))
	fmt.Println(kthGrammar(10, 2))
	fmt.Println(kthGrammar(10, 3))
	fmt.Println(kthGrammar(10, 4))
	fmt.Println(kthGrammar(10, 5))
	fmt.Println(kthGrammar(10, 6))
	fmt.Println(kthGrammar(10, 7))
	fmt.Println(kthGrammar(10, 8))
	fmt.Println(kthGrammar(30, 434991989))
}
