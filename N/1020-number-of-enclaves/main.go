package main

func numEnclaves(grid [][]int) int {
	cords := [][]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

	var clearDFS func(grid [][]int, x, y int)
	clearDFS = func(grid [][]int, x, y int) {
		grid[x][y] = 0
		for _, cord := range cords {
			nx, ny := x+cord[0], y+cord[1]
			if nx >= 0 && nx < len(grid) && ny >= 0 && ny < len(grid[0]) && grid[nx][ny] == 1 {
				clearDFS(grid, nx, ny)
			}
		}
	}
	for i := 0; i < len(grid); i++ {
		if grid[i][0] == 1 {
			clearDFS(grid, i, 0)
		}
		if grid[i][len(grid[0])-1] == 1 {
			clearDFS(grid, i, len(grid[0])-1)
		}
	}
	for j := 0; j < len(grid[0]); j++ {
		if grid[0][j] == 1 {
			clearDFS(grid, 0, j)
		}
		if grid[len(grid)-1][j] == 1 {
			clearDFS(grid, len(grid)-1, j)
		}
	}
	cnt := 0
	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[0]); j++ {
			if grid[i][j] == 1 {
				cnt++
			}
		}
	}
	return cnt
}
