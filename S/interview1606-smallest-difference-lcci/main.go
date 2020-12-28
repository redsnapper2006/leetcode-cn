package main

import (
	"fmt"
	"sort"
)

func smallestDifference(a []int, b []int) int {
	sort.Ints(a)
	sort.Ints(b)
	idxA, idxB := 0, 0
	ret := 1<<31 - 1
	for idxA < len(a) && idxB < len(b) {
		diff := a[idxA] - b[idxB]
		if diff < 0 {
			diff = -diff
		}
		if diff < ret {
			ret = diff
		}
		if a[idxA] < b[idxB] {
			idxA++
		} else {
			idxB++
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
