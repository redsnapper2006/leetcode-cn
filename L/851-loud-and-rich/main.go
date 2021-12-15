package main

import "fmt"

func loudAndRich(richer [][]int, quiet []int) []int {
	m := map[int][]int{}
	for _, v := range richer {
		_, ok := m[v[1]]
		if !ok {
			m[v[1]] = []int{}
		}
		m[v[1]] = append(m[v[1]], v[0])
	}

	v := map[int]int{}
	var dfs func(m map[int][]int, v map[int]int, quiet []int, idx int) int
	dfs = func(m map[int][]int, v map[int]int, quiet []int, idx int) int {
		r, ok := v[idx]
		if ok {
			return r
		}
		q := quiet[idx]
		for _, c := range m[idx] {
			nq := dfs(m, v, quiet, c)
			if q > nq {
				q = nq
			}
		}
		v[idx] = q
		return q
	}

	qm := map[int]int{}
	for i := 0; i < len(quiet); i++ {
		qm[quiet[i]] = i
		dfs(m, v, quiet, i)
	}

	retArr := []int{}
	for i := 0; i < len(quiet); i++ {
		retArr = append(retArr, qm[v[i]])
	}
	return retArr
}

func main() {
	fmt.Println(loudAndRich([][]int{{1, 0}, {2, 1}, {3, 1}, {3, 7}, {4, 3}, {5, 3}, {6, 3}}, []int{3, 2, 5, 4, 6, 1, 7, 0}))
}
