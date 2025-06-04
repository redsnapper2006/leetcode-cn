package main

import "fmt"

func numberOfMatches(n int) int {
	sum := 0
	for n > 1 {
		m := n % 2
		n /= 2
		sum += n
		n += m
	}

	return sum
}

func main() {
	fmt.Println()
}
