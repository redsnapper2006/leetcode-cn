package main

import (
	"fmt"
	"sort"
)

type Arr [][]int

func (ss Arr) Len() int {
	return len(ss)
}

func (ss Arr) Swap(i, j int) {
	ss[i], ss[j] = ss[j], ss[i]
}

func (ss Arr) Less(i, j int) bool {
	if ss[i][0] == ss[j][0] {
		return ss[i][1] > ss[j][1]
	}

	return ss[i][0] < ss[j][0]
}

func removeCoveredIntervals(intervals [][]int) int {
	ret := 0
	sort.Sort(Arr(intervals))
	base := intervals[0][1]
	for i := 1; i < len(intervals); i++ {
		// fmt.Println(i, base)
		if intervals[i][1] <= base {
			ret++
		} else {
			base = intervals[i][1]
		}
	}

	return len(intervals) - ret
}

func main() {
	fmt.Println()
}
