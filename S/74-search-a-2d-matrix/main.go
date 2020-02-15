package main

import (
	"fmt"
)

func searchMatrix(matrix [][]int, target int) bool {
	row := len(matrix)
	if row == 0 {
		return false
	}
	col := len(matrix[0])
	if col == 0 {
		return false
	}
	r, c := row-1, 0
	for r >= 0 && c < col {
		if matrix[r][c] < target {
			c++
		} else if matrix[r][c] > target {
			r--
		} else {
			return true
		}
	}
	return false
}

func main() {
	fmt.Println(searchMatrix([][]int{{1, 3, 5, 7},
		{10, 11, 16, 20},
		{23, 30, 34, 50}}, 13))
}
