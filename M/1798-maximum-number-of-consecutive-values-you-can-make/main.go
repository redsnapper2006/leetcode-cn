package main

import (
	"fmt"
	"sort"
)

func getMaximumConsecutive(coins []int) int {
	sum := 0
	sort.Ints(coins)

	for _, c := range coins {
		if sum+1 < c {
			return sum + 1
		}
		sum += c
	}
	return sum + 1
}

func main() {
	fmt.Println(getMaximumConsecutive([]int{1, 4, 10, 3, 1}))
}
