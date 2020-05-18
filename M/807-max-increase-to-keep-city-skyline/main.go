package main

import "fmt"

func maxIncreaseKeepingSkyline(grid [][]int) int {
	rowMax := make([]int, len(grid))
	colMax := make([]int, len(grid[0]))

	for i := 0; i < len(grid); i++ {
		max := grid[i][0]
		for j := 1; j < len(grid[0]); j++ {
			if max < grid[i][j] {
				max = grid[i][j]
			}
		}
		rowMax[i] = max
	}

	for i := 0; i < len(grid[0]); i++ {
		max := grid[0][i]
		for j := 1; j < len(grid); j++ {
			if max < grid[j][i] {
				max = grid[j][i]
			}
		}
		colMax[i] = max
	}

	count := 0
	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[0]); j++ {
			m := rowMax[i]
			if m > colMax[j] {
				m = colMax[j]
			}
			count += m - grid[i][j]
		}
	}
	return count
}

func main() {
	fmt.Println("a")
}
