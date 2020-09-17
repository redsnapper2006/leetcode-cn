package main

import "fmt"

func luckyNumbers(matrix [][]int) []int {
	buf := []int{}
	for i := 0; i < len(matrix); i++ {
		min := 1<<31 - 1
		colIdx := -1
		for j := 0; j < len(matrix[0]); j++ {
			if matrix[i][j] < min {
				min = matrix[i][j]
				colIdx = j
			}
		}
		max := -1 << 31
		for m := 0; m < len(matrix); m++ {
			if matrix[m][colIdx] > max {
				max = matrix[m][colIdx]
			}
		}
		if min == max {
			buf = append(buf, min)
		}
	}
	return buf
}

func main() {
	fmt.Println("a")
}
