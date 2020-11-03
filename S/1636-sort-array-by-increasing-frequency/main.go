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
	if p[i][0] != p[j][0] {
		return p[i][0] < p[j][0]
	}
	return p[i][1] > p[j][1]
}

func frequencySort(nums []int) []int {
	M := map[int]int{}
	for _, n := range nums {
		M[n]++
	}
	var buf [][]int
	for k, v := range M {
		buf = append(buf, []int{v, k})
	}
	sort.Sort(CordArrSlice(buf))
	var ret []int
	for _, v := range buf {
		for i := 0; i < v[0]; i++ {
			ret = append(ret, v[1])
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
