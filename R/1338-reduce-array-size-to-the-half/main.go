package main

import (
	"fmt"
	"sort"
)

func minSetSize(arr []int) int {
	m := map[int]int{}
	for i := 0; i < len(arr); i++ {
		m[arr[i]]++
	}
	remain := len(arr)
	c := 0
	count := map[int]int{}
	for _, v := range m {
		count[v]++
	}
	var candi []int
	for k := range count {
		candi = append(candi, k)
	}
	sort.Ints(candi)
	for i := len(candi) - 1; i >= 0; i-- {
		v := count[candi[i]]
		for j := 0; j < v; j++ {
			remain -= candi[i]
			c++
			if remain <= len(arr)/2 {
				return c
			}
		}
	}

	return c
}

func main() {
	fmt.Println("a")
}
