package main

import "fmt"

func countSubIslands(grid1 [][]int, grid2 [][]int) int {

	var dfs func(grid1 [][]int, grid2 [][]int, x, y int, status bool) bool
	dfs = func(grid1 [][]int, grid2 [][]int, x, y int, status bool) bool {
		curStatus := status && grid1[x][y] == 1
		grid2[x][y] = 0
		cord := [][]int{{0, -1}, {0, 1}, {1, 0}, {-1, 0}}
		for i := 0; i < len(cord); i++ {
			newx, newy := x+cord[i][0], y+cord[i][1]
			if newx >= 0 && newx < len(grid2) && newy >= 0 && newy < len(grid2[0]) && grid2[newx][newy] == 1 {
				r := dfs(grid1, grid2, newx, newy, curStatus)
				curStatus = curStatus && r
			}
		}
		return curStatus
	}
	ret := 0
	for i := 0; i < len(grid2); i++ {
		for j := 0; j < len(grid2[0]); j++ {
			if grid2[i][j] == 1 {
				r := dfs(grid1, grid2, i, j, true)
				if r {
					ret++
				}
			}
		}
	}
	return ret
}

func main() {
	fmt.Println(countSubIslands([][]int{{1, 1, 1, 0, 0}, {0, 1, 1, 1, 1}, {0, 0, 0, 0, 0}, {1, 0, 0, 0, 0}, {1, 1, 0, 1, 1}},
		[][]int{{1, 1, 1, 0, 0}, {0, 0, 1, 1, 1}, {0, 1, 0, 0, 0}, {1, 0, 1, 1, 0}, {0, 1, 0, 1, 0}}))
}
