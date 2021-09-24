package main

import (
	"fmt"
	"sort"
)

type Arr [][]int

func (p Arr) Len() int {
	return len(p)
}

func (p Arr) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p Arr) Less(i, j int) bool {
	if p[i][0] == p[j][0] {
		return p[i][1] > p[j][1]
	}

	return p[i][0] < p[j][0]
}

func removeCoveredIntervals(intervals [][]int) int {
	sort.Sort(Arr(intervals))

	base := intervals[0][1]
	cnt := 1
	for i := 1; i < len(intervals); i++ {
		if intervals[i][1] > base {
			base = intervals[i][1]
			cnt++
		}
	}
	return cnt
}

func main() {
	fmt.Println()
}
