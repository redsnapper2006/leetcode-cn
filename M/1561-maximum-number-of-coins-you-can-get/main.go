package main

import (
	"fmt"
	"sort"
)

func maxCoins(piles []int) int {
	sort.Ints(piles)

	sum := 0
	for len(piles) > 0 {
		sum += piles[len(piles)-2]
		piles = piles[1 : len(piles)-2]
	}
	return sum
}

func main() {
	fmt.Println()
}
