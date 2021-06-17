package main

import (
	"fmt"
)

func numJewelsInStones(J string, S string) int {
	M := make(map[byte]int)
	for i := 0; i < len(J); i++ {
		M[J[i]] = 1
	}

	c := 0
	for i := 0; i < len(S); i++ {
		if _, ok := M[S[i]]; ok {
			c++
		}
	}
	return c
}

func main() {
	fmt.Println("a")
}
