package main

import "fmt"

func gardenNoAdj(N int, paths [][]int) []int {
	R := map[int][]int{}
	for i := 0; i < len(paths); i++ {
		p := paths[i]
		_, ok := R[p[0]]
		if !ok {
			R[p[0]] = []int{}
		}
		R[p[0]] = append(R[p[0]], p[1])
		_, ok2 := R[p[1]]
		if !ok2 {
			R[p[1]] = []int{}
		}
		R[p[1]] = append(R[p[1]], p[0])
	}

	M := make([]int, N)
	for i := 1; i <= N; i++ {
		rset := R[i]
		mark := make([]int, 4)
		for j := 0; j < len(rset); j++ {
			if M[rset[j]-1] == 0 {
				continue
			}
			mark[M[rset[j]-1]-1] = 1
		}
		idx := -1
		for m := 0; m < 4; m++ {
			if mark[m] == 0 {
				idx = m
				break
			}
		}
		M[i-1] = idx + 1
	}
	return M
}

func main() {
	fmt.Println("a")
}
