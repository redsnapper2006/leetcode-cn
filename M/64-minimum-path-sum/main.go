package main

import (
	"fmt"
)

func minPathSum(grid [][]int) int {
	row := len(grid)
	col := len(grid[0])
	tbl := make([][]int, row)
	for i := 0; i < row; i++ {
		tbl[i] = make([]int, col)
	}
	tbl[0][0] = grid[0][0]
	for i := 1; i < col; i++ {
		tbl[0][i] = grid[0][i] + tbl[0][i-1]
	}
	for i := 1; i < row; i++ {
		tbl[i][0] = grid[i][0] + tbl[i-1][0]
	}
	for i := 1; i < col; i++ {
		for j := 1; j < row; j++ {
			if tbl[j-1][i] > tbl[j][i-1] {
				tbl[j][i] = tbl[j][i-1] + grid[j][i]
			} else {
				tbl[j][i] = tbl[j-1][i] + grid[j][i]
			}
		}
	}
	return tbl[row-1][col-1]
}

func main() {
	fmt.Println(minPathSum([][]int{{1, 3, 1},
		{1, 5, 1},
		{4, 2, 1}}))
}
