package main

import "fmt"

func maxAreaOfIsland(grid [][]int) int {
	var recur func(grid [][]int, r, c int) int
	recur = func(grid [][]int, r, c int) int {
		grid[r][c] = 0
		ret := 1
		if r > 0 && grid[r-1][c] == 1 {
			ret += recur(grid, r-1, c)
		}
		if c > 0 && grid[r][c-1] == 1 {
			ret += recur(grid, r, c-1)
		}
		if r < len(grid)-1 && grid[r+1][c] == 1 {
			ret += recur(grid, r+1, c)
		}
		if c < len(grid[0])-1 && grid[r][c+1] == 1 {
			ret += recur(grid, r, c+1)
		}
		return ret
	}
	max := 0
	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[0]); j++ {
			if grid[i][j] == 1 {
				ret := recur(grid, i, j)
				if ret > max {
					max = ret
				}
			}
		}
	}
	return max
}

func main() {
	fmt.Println("a")
}
