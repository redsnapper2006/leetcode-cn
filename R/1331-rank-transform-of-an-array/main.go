package main

import (
	"fmt"
	"sort"
)

func arrayRankTransform(arr []int) []int {
	m := map[int]int{}
	for _, v := range arr {
		m[v]++
	}
	var buf []int
	for k := range m {
		buf = append(buf, k)
	}
	sort.Ints(buf)
	ms := map[int]int{}
	for i, v := range buf {
		ms[v] = i + 1
	}
	var ret []int
	for _, v := range arr {
		ret = append(ret, ms[v])
	}
	return ret
}

func main() {
	fmt.Println("a")
}
