package main

import (
	"fmt"
	"sort"
)

// RelativeSortArr is RelativeSortArr
type RelativeSortArr struct {
	Arr []int
	M   map[int]int
}

func (p RelativeSortArr) Len() int {
	return len(p.Arr)
}

func (p RelativeSortArr) Swap(i, j int) {
	p.Arr[i], p.Arr[j] = p.Arr[j], p.Arr[i]
}

func (p RelativeSortArr) Less(i, j int) bool {
	i1, ok1 := p.M[p.Arr[i]]
	i2, ok2 := p.M[p.Arr[j]]
	if ok1 && !ok2 {
		return true
	}
	if !ok1 && ok2 {
		return false
	}
	if ok1 && ok2 {
		return i1 < i2
	}
	return p.Arr[i] < p.Arr[j]
}

func relativeSortArray(arr1 []int, arr2 []int) []int {
	M := map[int]int{}
	for i, v := range arr2 {
		M[v] = i
	}
	p := RelativeSortArr{Arr: arr1, M: M}
	sort.Sort(p)
	return p.Arr

}

func main() {
	fmt.Println("a")
}
