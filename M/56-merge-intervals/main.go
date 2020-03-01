package main

import (
	"fmt"
	"sort"
)

type ArraySlice [][]int

func (p ArraySlice) Len() int {
	return len(p)
}

func (p ArraySlice) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p ArraySlice) Less(i, j int) bool {
	return p[i][0] < p[j][0]
}

func merge(intervals [][]int) [][]int {
	if len(intervals) <= 1 {
		return intervals
	}
	sort.Sort(ArraySlice(intervals))
	base := intervals[0]
	var r [][]int
	for i := 1; i < len(intervals); i++ {
		if intervals[i][0] <= base[1] {
			if intervals[i][1] > base[1] {
				base[1] = intervals[i][1]
			}
		} else {
			r = append(r, base)
			base = intervals[i]
		}
	}
	r = append(r, base)

	return r
}

func main() {
	fmt.Println("a")
}
