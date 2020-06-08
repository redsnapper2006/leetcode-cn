package main

import "fmt"

func searchMatrix(matrix [][]int, target int) bool {
	r, c := len(matrix)-1, 0

	for r >= 0 && c < len(matrix[0]) {
		if matrix[r][c] == target {
			return true
		} else if matrix[r][c] > target {
			r--
		} else {
			c++
		}
	}

	return false
}

func main() {
	fmt.Println("a")
}
