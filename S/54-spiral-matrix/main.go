package main

import (
	"fmt"
)

func spiralOrder(matrix [][]int) []int {
	if len(matrix) == 0 || len(matrix[0]) == 0 {
		return []int{}
	}

	var buf []int
	iDirection := 0
	iRow := len(matrix)
	iCol := len(matrix[0])

	iR, iC := 0, 0
	for iRow > 0 && iCol > 0 {
		if iDirection == 0 {
			for i := 0; i < iCol; i++ {
				buf = append(buf, matrix[iR][iC])
				iC++
			}
			iDirection = 1
			iRow--
			iR++
			iC--
		} else if iDirection == 1 {
			for i := 0; i < iRow; i++ {
				buf = append(buf, matrix[iR][iC])
				iR++
			}
			iDirection = 2
			iCol--
			iC--
			iR--
		} else if iDirection == 2 {
			for i := 0; i < iCol; i++ {
				buf = append(buf, matrix[iR][iC])
				iC--
			}
			iDirection = 3
			iRow--
			iR--
			iC++
		} else if iDirection == 3 {
			for i := 0; i < iRow; i++ {
				buf = append(buf, matrix[iR][iC])
				iR--
			}
			iDirection = 0
			iCol--
			iC++
			iR++
		}
	}

	return buf
}

func main() {
	fmt.Println(spiralOrder([][]int{{1, 2, 3, 4},
		{5, 6, 7, 8},
		{9, 10, 11, 12}}))
}
