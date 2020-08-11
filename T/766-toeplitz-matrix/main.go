package main

import (
	"fmt"
)

func isToeplitzMatrix(matrix [][]int) bool {

	buf := make([]int, len(matrix)+len(matrix[0])-1)
	for i := 0; i < len(matrix); i++ {
		buf[i+len(matrix[0])-1] = matrix[i][0]
	}
	for j := 0; j < len(matrix[0]); j++ {
		buf[-j+len(matrix[0])-1] = matrix[0][j]
	}
	for i := 1; i < len(matrix); i++ {
		for j := 1; j < len(matrix[0]); j++ {
			if buf[i-j+len(matrix[0])-1] != matrix[i][j] {
				return false
			}
		}
	}
	return true
}

func main() {
	fmt.Println("a")
}
