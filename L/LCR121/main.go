package main

import "fmt"

func findNumberIn2DArray(matrix [][]int, target int) bool {
	row, col := len(matrix)-1, 0

	for row >= 0 && col < len(matrix[0]) {
		if matrix[row][col] == target {
			return true
		} else if matrix[row][col] < target {
			col++
		} else {
			row--
		}
	}
	return false
}

func main() {
	fmt.Println("a")
}
