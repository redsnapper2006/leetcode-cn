package main

import (
	"fmt"
)

func groupThePeople(groupSizes []int) [][]int {
	M := map[int][]int{}
	for i, v := range groupSizes {
		_, ok := M[v]
		if !ok {
			M[v] = []int{}

		}
		M[v] = append(M[v], i)
	}
	var ret [][]int
	for k, v := range M {
		idx := 0
		for idx < len(v) {
			var t []int
			for i := 0; i < k; i++ {
				t = append(t, v[idx+i])
			}
			idx += k
			ret = append(ret, t)
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
