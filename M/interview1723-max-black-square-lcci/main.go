package main

import (
	"fmt"
)

func findSquare(matrix [][]int) []int {
	buf := make([][][]int, len(matrix))
	for i := 0; i < len(matrix); i++ {
		buf[i] = make([][]int, len(matrix[0]))
	}
	max := 0
	ridx, cidx := -1, -1
	for i := len(matrix) - 1; i >= 0; i-- {
		for j := len(matrix[0]) - 1; j >= 0; j-- {
			if matrix[i][j] == 1 {
				buf[i][j] = []int{0, 0}
				continue
			}

			r, c := 0, 0
			if i < len(matrix)-1 {
				c = buf[i+1][j][1]
			}
			if j < len(matrix[0])-1 {
				r = buf[i][j+1][0]
			}
			buf[i][j] = []int{r + 1, c + 1}

			size := r + 1
			if size > c+1 {
				size = c + 1
			}
			ret := 1
			for m := size; m >= 1; m-- {
				if buf[i+m-1][j][0] >= m && buf[i][j+m-1][1] >= m {
					ret = m
					break
				}
			}
			if max < ret || (max == ret && (ridx > i || cidx > j)) {
				ridx = i
				cidx = j
				max = ret
			}
		}
	}

	if max == 0 {
		return nil
	}
	return []int{ridx, cidx, max}
}

func main() {
	fmt.Println(findSquare([][]int{{1, 1, 1, 0, 1, 1, 0, 1, 0, 0}, {0, 1, 0, 1, 1, 0, 0, 0, 1, 1}, {0, 0, 1, 1, 0, 0, 1, 1, 1, 0}, {0, 1, 1, 1, 0, 1, 0, 0, 1, 0}, {1, 1, 0, 1, 1, 0, 1, 0, 0, 1}, {0, 1, 1, 0, 0, 0, 0, 1, 1, 0}, {1, 0, 0, 0, 0, 1, 1, 1, 1, 1}, {1, 0, 1, 0, 1, 0, 0, 0, 1, 0}, {1, 1, 1, 1, 0, 1, 0, 1, 0, 0}, {0, 0, 0, 0, 0, 0, 0, 1, 1, 0}}))
}
