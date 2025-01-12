package main

import (
	"fmt"
)

func printNumbers(n int) []int {
	max := 1
	for i := 0; i < n; i++ {
		max *= 10
	}
	var r []int
	for i := 1; i < max; i++ {
		r = append(r, i)
	}
	return r
}

func main() {
	fmt.Println("a")
}
