package main

import "fmt"

func allPathsSourceTarget(graph [][]int) [][]int {
	ret := [][]int{}

	var dfs func(candi []int)
	dfs = func(candi []int) {
		c := candi[len(candi)-1]
		for _, n := range graph[c] {
			t := make([]int, len(candi))
			copy(t, candi)
			t = append(t, n)
			if n == len(graph)-1 {
				ret = append(ret, t)
				continue
			}
			dfs(t)
		}
	}

	dfs([]int{0})
	return ret
}

func main() {
	fmt.Println()
}
