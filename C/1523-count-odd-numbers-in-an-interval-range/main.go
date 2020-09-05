package main

import "fmt"

func countOdds(low int, high int) int {
	c := (high - low) / 2
	if low%2 == 1 || high%2 == 1 {
		c++
	}
	return c
}

func main() {
	fmt.Println("a")
}
