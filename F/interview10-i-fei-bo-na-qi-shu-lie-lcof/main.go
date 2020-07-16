package main

import "fmt"

func fib(n int) int {
	if n < 2 {
		return n
	}
	a, b := 0, 1
	i := 2
	for i <= n {
		a, b = b, (a+b)%1000000007
		i++
	}
	return b
}

func main() {
	fmt.Println("a")
}
