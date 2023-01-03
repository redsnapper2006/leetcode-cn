package main

import (
	"fmt"
)



func countGoodRectangles(rectangles [][]int) int {
	M := map[int]int{}
	for _, r := range rectangles {
		min := r[0]
		if min > r[1] {
			min = r[1]
		}
		M[min]++
	}
	max := -1
	for k := range M {
		if k > max {
			max = k
		}
	}
	return M[max]
}

func main() {
	fmt.Println()
}
