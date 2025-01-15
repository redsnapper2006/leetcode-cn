package main

import "fmt"

type NumMatrix struct {
	Sum [][]int
}

func Constructor(matrix [][]int) NumMatrix {
	buf := make([][]int, len(matrix))
	for i := 0; i < len(matrix); i++ {
		buf[i] = make([]int, len(matrix[0]))
		sum := 0
		for j := 0; j < len(matrix[0]); j++ {
			sum += matrix[i][j]
			prev := 0
			if i > 0 {
				prev = buf[i-1][j]
			}
			buf[i][j] = prev + sum
		}
	}
	return NumMatrix{Sum: buf}
}

func (this *NumMatrix) SumRegion(row1 int, col1 int, row2 int, col2 int) int {
	total := this.Sum[row2][col2]
	up := 0
	if row1 > 0 {
		up = this.Sum[row1-1][col2]
	}
	left := 0
	if col1 > 0 {
		left = this.Sum[row2][col1-1]
	}

	leftup := 0
	if row1 > 0 && col1 > 0 {
		leftup = this.Sum[row1-1][col1-1]
	}
	return total - left - up + leftup
}

func main() {
	fmt.Println()
}
