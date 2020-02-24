package main

import "fmt"

func fib(N int) int {

	var recur func(N int, cache map[int]int) int
	recur = func(N int, cache map[int]int) int {
		if N <= 1 {
			return N
		}
		v, ok := cache[N]
		if ok {

			fmt.Println("hit", N)
			return v
		} else {
			t := recur(N-1, cache) + recur(N-2, cache)
			cache[N] = t
			return t
		}
	}
	M := make(map[int]int)
	return recur(N, M)
}

func main() {
	fmt.Println(fib(4))
}
