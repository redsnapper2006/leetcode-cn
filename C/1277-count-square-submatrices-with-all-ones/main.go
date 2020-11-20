package main

import "fmt"

func countSquares(matrix [][]int) int {
	buf := make([][]int, len(matrix))
	for i := 0; i < len(matrix); i++ {
		buf[i] = make([]int, len(matrix[0]))
	}
	ret := 0
	for i := 0; i < len(matrix); i++ {
		for j := 0; j < len(matrix[0]); j++ {
			if matrix[i][j] == 0 {
				buf[i][j] = 0
				continue
			}
			row, col, cross := 0, 0, 0
			if i > 0 {
				row = buf[i-1][j]
			}
			if j > 0 {
				col = buf[i][j-1]
			}
			if i > 0 && j > 0 {
				cross = buf[i-1][j-1]
			}
			min := row
			if col < min {
				min = col
			}
			if cross < min {
				min = cross
			}
			buf[i][j] = min + 1
			ret += buf[i][j]
		}
	}

	return ret
}

func main() {
	fmt.Println()
}
