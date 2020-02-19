package main

import (
	"fmt"
)

func findDiagonalOrder(matrix [][]int) []int {
	if len(matrix) == 0 || len(matrix[0]) == 0 {
		return []int{}
	}
	r := len(matrix)
	c := len(matrix[0])

	var buf []int
	for i := 0; i < r+c-1; i++ {
		if i%2 == 0 {
			sRow := i
			if i > r-1 {
				sRow = r - 1
			}
			sCol := i - sRow
			for sCol < c && sRow >= 0 {
				buf = append(buf, matrix[sRow][sCol])
				sRow--
				sCol++
			}
		} else {
			sCol := i
			if i > c-1 {
				sCol = c - 1
			}
			sRow := i - sCol
			for sCol >= 0 && sRow < r {
				buf = append(buf, matrix[sRow][sCol])
				sCol--
				sRow++
			}
		}
	}
	return buf
}

func main() {

	fmt.Println(findDiagonalOrder([][]int{{1, 2, 3},
		{4, 5, 6},
		{7, 8, 9}}))
}
