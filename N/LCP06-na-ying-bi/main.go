package main

import "fmt"

func minCount(coins []int) int {
	sum := 0
	for i := 0; i < len(coins); i++ {
		sum += (coins[i] + 1) / 2
	}

	return sum
}

func main() {
	fmt.Println("a")
}
