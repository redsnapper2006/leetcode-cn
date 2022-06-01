package main

import (
	"fmt"
	"sort"
)

func makesquare(matchsticks []int) bool {
	if len(matchsticks) < 4 {
		return false
	}
	sum := 0
	for i := 0; i < len(matchsticks); i++ {
		sum += matchsticks[i]
	}
	if sum%4 != 0 {
		return false
	}
	sort.Sort(sort.Reverse(sort.IntSlice(matchsticks)))

	edges := [4]int{}
	var dfs func(idx int) bool
	dfs = func(idx int) bool {
		if idx == len(matchsticks) {
			return true
		}
		for i := range edges {
			edges[i] += matchsticks[idx]
			for edges[i] <= sum/4 && dfs(idx+1) {
				return true
			}
			edges[i] -= matchsticks[idx]
		}
		return false
	}
	return dfs(0)
}

func main() {
	fmt.Println(makesquare([]int{5, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 3}))
}
