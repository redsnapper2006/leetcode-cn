package main

import "fmt"

func game(guess []int, answer []int) int {
	c := 0
	for i := 0; i < len(guess); i++ {
		if guess[i] == answer[i] {
			c++
		}
	}
	return c
}

func main() {
	fmt.Println("a")
}
