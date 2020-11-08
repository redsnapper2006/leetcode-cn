package main

import (
	"fmt"
	"sort"
)

func maxWidthOfVerticalArea(points [][]int) int {
	xm := map[int]int{}
	for _, v := range points {
		xm[v[0]]++
	}
	var cord []int
	for k := range xm {
		cord = append(cord, k)
	}
	if len(cord) <= 1 {
		return 0
	}
	sort.Ints(cord)

	max := -1
	for i := 1; i < len(cord); i++ {
		if cord[i]-cord[i-1] > max {
			max = cord[i] - cord[i-1]
		}
	}
	return max
}

func main() {
	fmt.Println()
}
