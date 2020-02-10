package main

import (
	"fmt"
)

func setZeroes(matrix [][]int) {
	zeroRow, zeroCol := false, false
	for i := 0; i < len(matrix); i++ {
		if matrix[i][0] == 0 {
			zeroCol = true
			break
		}
	}
	for i := 0; i < len(matrix[0]); i++ {
		if matrix[0][i] == 0 {
			zeroRow = true
			break
		}
	}

	for i := 1; i < len(matrix); i++ {
		for j := 1; j < len(matrix[0]); j++ {
			if matrix[i][j] == 0 {
				matrix[i][0] = 0
				matrix[0][j] = 0
			}
		}
	}
	for i := 1; i < len(matrix); i++ {
		if matrix[i][0] == 0 {
			for j := 1; j < len(matrix[0]); j++ {
				matrix[i][j] = 0
			}
		}
	}
	for i := 1; i < len(matrix[0]); i++ {
		if matrix[0][i] == 0 {
			for j := 1; j < len(matrix); j++ {
				matrix[j][i] = 0
			}
		}
	}
	if zeroRow {
		for i := 0; i < len(matrix[0]); i++ {
			matrix[0][i] = 0
		}
	}
	if zeroCol {
		for i := 0; i < len(matrix); i++ {
			matrix[i][0] = 0
		}
	}
}

func main() {
	// a := [][]int{{1, 1, 1},
	// 	{1, 0, 1},
	// 	{1, 1, 1}}
	a := [][]int{{0, 1, 2, 0},
		{3, 4, 5, 2},
		{1, 3, 1, 5}}
	setZeroes(a)
	fmt.Println(a)
}
