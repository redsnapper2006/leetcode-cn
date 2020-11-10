package main

import (
	"fmt"
	"sort"
)

func diagonalSort(mat [][]int) [][]int {
	RN, CN := len(mat), len(mat[0])
	for i := RN - 1; i >= 0; i-- {
		var buf []int
		colIdx := 0
		for j := i; j < RN && colIdx < CN; j, colIdx = j+1, colIdx+1 {
			buf = append(buf, mat[j][colIdx])
		}
		sort.Ints(buf)
		colIdx = 0
		idx := 0
		for j := i; j < RN && colIdx < CN; j, colIdx, idx = j+1, colIdx+1, idx+1 {
			mat[j][colIdx] = buf[idx]
		}
	}

	for i := 1; i < CN; i++ {
		rowIdx := 0
		var buf []int
		for j := i; j < CN && rowIdx < RN; j, rowIdx = j+1, rowIdx+1 {
			buf = append(buf, mat[rowIdx][j])
		}
		sort.Ints(buf)
		rowIdx = 0
		idx := 0
		for j := i; j < CN && rowIdx < RN; j, rowIdx, idx = j+1, rowIdx+1, idx+1 {
			mat[rowIdx][j] = buf[idx]
		}
	}
	return mat
}

func main() {
	fmt.Println()
}
