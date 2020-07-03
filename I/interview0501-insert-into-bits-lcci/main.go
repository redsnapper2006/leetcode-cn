package main

import "fmt"

func insertBits(N int, M int, i int, j int) int {
	for m := i; m <= j; m++ {
		if M&(1<<(m-i)) > 0 {
			N = N | (1 << m)
		} else {
			N = N & (0xFFFFFFFF ^ (1 << m))
		}
	}
	return N
}

func main() {
	fmt.Println("a")
}
