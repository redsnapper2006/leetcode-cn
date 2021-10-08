package main

import (
	"fmt"
	"sort"
)

func findOriginalArray(changed []int) []int {
	if len(changed)%2 != 0 {
		return []int{}
	}

	sort.Ints(changed)

	ret := []int{}
	sum := map[int]int{}
	for _, v := range changed {
		if sum[v] > 0 {
			sum[v]--
			continue
		}
		sum[v*2]++

		ret = append(ret, v)
	}
	for _, v := range sum {
		if v > 0 {
			return []int{}
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
