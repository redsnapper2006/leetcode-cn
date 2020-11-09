package main

import (
	"fmt"
	"sort"
)

type CordArrSlice [][]int

func (p CordArrSlice) Len() int {
	return len(p)
}

func (p CordArrSlice) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p CordArrSlice) Less(i, j int) bool {
	return p[i][0] < p[j][0]
}

func kClosest(points [][]int, K int) [][]int {
	var buf [][]int
	for i, p := range points {
		buf = append(buf, []int{p[0]*p[0] + p[1]*p[1], i})
	}
	sort.Sort(CordArrSlice(buf))
	var ret [][]int
	for i := 0; i < K; i++ {
		ret = append(ret, points[buf[i][1]])
	}
	return ret
}

func main() {
	fmt.Println()
}
