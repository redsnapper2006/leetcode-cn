package main

import (
	"fmt"
	"sort"
)

type BitSlice struct {
	BIT []int
	Arr []int
}

func (p BitSlice) Len() int {
	return len(p.Arr)
}

func (p BitSlice) Swap(i, j int) {
	p.Arr[i], p.Arr[j] = p.Arr[j], p.Arr[i]
}

func (p BitSlice) Less(i, j int) bool {
	if p.BIT[p.Arr[i]] != p.BIT[p.Arr[j]] {
		return p.BIT[p.Arr[i]] < p.BIT[p.Arr[j]]
	}
	return p.Arr[i] < p.Arr[j]
}

func sortByBits(arr []int) []int {
	bit := make([]int, 10001)
	for i := 1; i <= 10000; i++ {
		bit[i] = bit[i>>1] + (i & 1)
	}
	p := BitSlice{BIT: bit, Arr: arr}
	sort.Sort(p)
	return p.Arr
}

func main() {
	fmt.Println("a")
}
