package main

import "fmt"

func maxAreaOfIsland(grid [][]int) int {
	max := 0
	var dfs func(grid [][]int, x, y int) int
	dfs = func(grid [][]int, x, y int) int {

		ret := 1
		grid[x][y] = 0
		cords := [][]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
		for _, cord := range cords {
			nx, ny := x+cord[0], y+cord[1]
			if nx >= 0 && nx < len(grid) && ny >= 0 && ny < len(grid[0]) && grid[nx][ny] == 1 {

				ret += dfs(grid, nx, ny)
			}
		}
		return ret
	}

	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[0]); j++ {
			if grid[i][j] == 1 {
				v := dfs(grid, i, j)
				if v > max {
					max = v
				}
			}
		}
	}
	return max
}

func main() {
	fmt.Println()
}
