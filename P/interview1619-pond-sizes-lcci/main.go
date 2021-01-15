package main

import (
	"fmt"
	"sort"
)

func pondSizes(land [][]int) []int {
	var dfs func(land [][]int, r, c int) int
	dfs = func(land [][]int, r, c int) int {
		land[r][c] = -1
		size := 1
		if r > 0 && c > 0 && land[r-1][c-1] == 0 {
			size += dfs(land, r-1, c-1)
		}
		if r > 0 && land[r-1][c] == 0 {
			size += dfs(land, r-1, c)
		}
		if r > 0 && c < len(land[0])-1 && land[r-1][c+1] == 0 {
			size += dfs(land, r-1, c+1)
		}
		if c > 0 && land[r][c-1] == 0 {
			size += dfs(land, r, c-1)
		}
		if c < len(land[0])-1 && land[r][c+1] == 0 {
			size += dfs(land, r, c+1)
		}
		if r < len(land)-1 && c > 0 && land[r+1][c-1] == 0 {
			size += dfs(land, r+1, c-1)
		}
		if r < len(land)-1 && land[r+1][c] == 0 {
			size += dfs(land, r+1, c)
		}
		if r < len(land)-1 && c < len(land[0])-1 && land[r+1][c+1] == 0 {
			size += dfs(land, r+1, c+1)
		}
		return size
	}
	var ret []int
	for i := 0; i < len(land); i++ {
		for j := 0; j < len(land[0]); j++ {
			if land[i][j] == 0 {
				ret = append(ret, dfs(land, i, j))
			}
		}
	}
	sort.Ints(ret)
	return ret
}

func main() {
	fmt.Println()
}
