package main

import (
	"fmt"
	"sort"
)

func subSort(array []int) []int {
	if len(array) <= 1 {
		return []int{-1, -1}
	}

	m := array[0]
	rIdx := -1
	for i := 1; i < len(array); i++ {
		if array[i] >= m {
			m = array[i]
		} else {
			rIdx = i
		}
	}

	lIdx := -1
	m = array[len(array)-1]
	for i := len(array) - 2; i >= 0; i-- {
		if array[i] <= m {
			m = array[i]
		} else {
			lIdx = i
		}
	}

	return []int{lIdx, rIdx}
}

func subSortV2(array []int) []int {
	buf := make([]int, len(array))
	copy(buf, array)
	sort.Ints(buf)
	s, e := 0, len(array)-1
	for s < len(array) && buf[s] == array[s] {
		s++
	}
	if s >= len(array) {
		return []int{-1, -1}
	}
	for e >= 0 && buf[e] == array[e] {
		e--
	}
	return []int{s, e}
}

func main() {
	fmt.Println()
}
