package main

import (
	"fmt"
	"sort"
)

func minCostToMoveChips(position []int) int {
	sort.Ints(position)
	var buf []int
	var count []int

	for i := 0; i < len(position); i++ {
		if i > 0 && position[i] == position[i-1] {
			continue
		}
		buf = append(buf, position[i])
	}
	candi := buf[0]
	c := 1
	for i := 1; i < len(position); i++ {
		if position[i] == candi {
			c++
		} else {
			count = append(count, c)
			c = 1
			candi = position[i]
		}
	}
	count = append(count, c)

	ret := make([]int, len(buf))
	for i := 0; i < len(buf); i++ {
		for j := i + 1; j < len(buf); j++ {
			m := (buf[j] - buf[i]) % 2
			ret[i] += m * count[j]
			ret[j] += m * count[i]
		}
	}

	min := 1<<31 - 1
	for _, v := range ret {
		if v < min {
			min = v
		}
	}
	return min
}

func main() {
	fmt.Println("a")
}
