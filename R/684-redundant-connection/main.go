package main

import "fmt"

func findRedundantConnection(edges [][]int) []int {
	N := len(edges)
	EM := map[int][]int{}
	for _, edge := range edges {
		s, e := edge[0], edge[1]
		EM[s] = append(EM[s], e)
		EM[e] = append(EM[e], s)
	}

	var dfs func(visit map[int]int, start int, n int) map[int]int
	dfs = func(visit map[int]int, start int, n int) map[int]int {
		dst := EM[n]
		for i := 0; i < len(dst); i++ {
			if len(visit) > 2 && dst[i] == start {
				return visit
			}
			_, ok := visit[dst[i]]
			if ok {
				continue
			}
			visit[dst[i]] = 1
			v := dfs(visit, start, dst[i])
			if v != nil {
				return v
			}
			delete(visit, dst[i])
		}
		return nil
	}

	for i := 1; i <= N; i++ {
		visit := map[int]int{i: 1}
		v := dfs(visit, i, i)
		if v != nil {
			for j := len(edges) - 1; j >= 0; j-- {
				_, ok1 := visit[edges[j][0]]
				_, ok2 := visit[edges[j][1]]
				if ok1 && ok2 {
					return edges[j]
				}
			}
		}
	}
	return nil
}

func main() {
	fmt.Println()
}
