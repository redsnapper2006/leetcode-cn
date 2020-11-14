package main

import (
	"fmt"
	"sort"
)

func maxSatisfaction(satisfaction []int) int {
	sort.Ints(satisfaction)
	sum := 0
	accum := 0
	max := 0
	for i := len(satisfaction) - 1; i >= 0; i-- {
		accum += sum + satisfaction[i]
		sum += satisfaction[i]
		if accum > max {
			max = accum
		}
	}
	return max
}

func main() {
	fmt.Println()
}
