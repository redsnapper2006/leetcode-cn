package main

import (
	"fmt"
	"sort"
)

func kWeakestRows(mat [][]int, k int) []int {
	buf := make([]int, len(mat))
	for i := 0; i < len(mat); i++ {
		s := 0
		for j := 0; j < len(mat[0]); j++ {
			s += mat[i][j]
		}
		buf[i] = s
	}
	carr := []int{}
	m := map[int][]int{}
	for i := 0; i < len(buf); i++ {
		_, ok := m[buf[i]]
		if !ok {
			m[buf[i]] = []int{}
			carr = append(carr, buf[i])
		}
		m[buf[i]] = append(m[buf[i]], i)
	}
	sort.Ints(carr)

	var r []int
	idx := 0
	for len(r) < k {
		arr := m[carr[idx]]
		sort.Ints(arr)
		r = append(r, arr...)
		idx++
	}

	return r[0:k]
}

func main() {
	fmt.Println("a")
}
