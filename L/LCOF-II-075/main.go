package main

import (
	"fmt"
	"sort"
)

type RINT struct {
	D []int
	M map[int]int
}

func (ri RINT) Len() int {
	return len(ri.D)
}

func (ri RINT) Swap(i, j int) {
	ri.D[i], ri.D[j] = ri.D[j], ri.D[i]
}

func (ri RINT) Less(i, j int) bool {
	v1, ok1 := ri.M[ri.D[i]]
	v2, ok2 := ri.M[ri.D[j]]

	if ok1 && ok2 {
		return v1 < v2
	} else if !ok1 && ok2 {
		return false
	} else if ok1 && !ok2 {
		return true
	}

	return ri.D[i] < ri.D[j]
}

func relativeSortArray(arr1 []int, arr2 []int) []int {
	m := map[int]int{}
	for i, v := range arr2 {
		m[v] = i + 1
	}
	ri := RINT{D: arr1, M: m}
	sort.Sort(ri)
	return ri.D
}

func main() {
	fmt.Println()
}
