package main

import "fmt"

func minOperations(n int) int {
	sum := 0
	c := n - 1
	for c > 0 {
		sum += c
		c -= 2
	}
	return sum
}

func main() {
	fmt.Println("a")
}
