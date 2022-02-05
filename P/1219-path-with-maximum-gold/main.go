package main

import "fmt"

func getMaximumGold(grid [][]int) int {
	max := 0
	var dfs func(grid [][]int, occu [][]int, r, c, sum int)
	dfs = func(grid [][]int, occu [][]int, r, c, sum int) {
		occu[r][c] = 1
		sum += grid[r][c]
		if sum > max {
			max = sum
		}

		cords := [][]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
		for _, cord := range cords {
			nr, nc := r+cord[0], c+cord[1]
			if nr >= 0 && nr < len(grid) && nc >= 0 && nc < len(grid[0]) && grid[nr][nc] > 0 && occu[nr][nc] == 0 {
				// hasNext = true
				dfs(grid, occu, nr, nc, sum)
			}
		}
		occu[r][c] = 0
	}

	buf := make([][]int, len(grid))
	for i := 0; i < len(grid); i++ {
		buf[i] = make([]int, len(grid[0]))
	}

	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[0]); j++ {
			if grid[i][j] > 0 {
				dfs(grid, buf, i, j, 0)
			}
		}
	}
	return max
}

func main() {
	fmt.Println(getMaximumGold([][]int{{0, 6, 0}, {5, 8, 7}, {0, 9, 0}}))
}
