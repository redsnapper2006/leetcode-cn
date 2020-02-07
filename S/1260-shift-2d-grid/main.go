package main

import (
	"fmt"
)

func shiftGrid(grid [][]int, k int) [][]int {
	if len(grid) == 0 {
		return grid
	}

	row := len(grid)
	col := len(grid[0])

	rowShift := k / col
	colShift := k % col

	ret := make([][]int, row)
	for i := 0; i < row; i++ {
		ret[i] = make([]int, col)
	}
	for i := 0; i < row; i++ {
		for j := 0; j < col; j++ {
			r := (i + rowShift)
			c := (j + colShift)
			if c >= col {
				c %= col
				r++
			}
			r %= row
			ret[r][c] = grid[i][j]
		}
	}
	return ret
}

func main() {
	fmt.Println(shiftGrid([][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}, 1))

}
