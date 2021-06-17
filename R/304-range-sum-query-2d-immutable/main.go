package main

import (
	"fmt"
)

type NumMatrix struct {
	BUF [][]int
}

func Constructor(matrix [][]int) NumMatrix {
	buf := make([][]int, len(matrix))
	for i := 0; i < len(matrix); i++ {
		buf[i] = make([]int, len(matrix[0]))
		sum := 0
		for j := 0; j < len(matrix[0]); j++ {
			sum += matrix[i][j]
			buf[i][j] = sum
		}
	}

	return NumMatrix{BUF: buf}
}

func (this *NumMatrix) SumRegion(row1 int, col1 int, row2 int, col2 int) int {
	ret := 0
	for i := row1; i <= row2; i++ {
		s := 0
		if col1 > 0 {
			s = this.BUF[i][col1-1]
		}
		ret += this.BUF[i][col2] - s
	}
	return ret
}

func main() {
	fmt.Println("a")
}
