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

	var r0start, r0end int = 0, col - 1

	for r0end > r0start {
		if matrix[0][(r0end+r0start+1)/2] > target {
			r0end = (r0end+r0start+1)/2 - 1
		} else if matrix[0][(r0end+r0start+1)/2] < target {
			r0start = (r0end+r0start+1)/2 + 1
		} else {
			break
		}
	}

	if r0end < 0 {
		r0end = 0
	}
	if r0start > col-1 {
		r0start = col - 1
	}

	for i := 0; i <= r0end; i++ {
		var c0start, c0end int = 0, row - 1

		for c0end > c0start {
			if matrix[(c0end+c0start+1)/2][i] > target {
				c0end = (c0end+c0start+1)/2 - 1
			} else if matrix[(c0end+c0start+1)/2][i] < target {
				c0start = (c0end+c0start+1)/2 + 1
			} else {
				return true
			}
		}
		if c0end < 0 {
			c0end = 0
		}
		if c0start > row-1 {
			c0start = row - 1
		}
		if matrix[c0end][i] == target || matrix[c0start][i] == target {
			return true
		}
	}

	return false
}

func main() {
	fmt.Println(searchMatrix([][]int{
		{1, 4, 7, 11, 15},
		{2, 5, 8, 12, 19},
		{3, 6, 9, 16, 22},
		{10, 13, 14, 17, 24},
		{18, 21, 23, 26, 30},
	}, 5))
	fmt.Println(searchMatrix([][]int{}, -5))
	fmt.Println(searchMatrix([][]int{{}}, -5))
}
