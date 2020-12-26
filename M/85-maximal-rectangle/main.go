package main

import "fmt"

func maximalRectangle(matrix [][]byte) int {
	buf := make([][][]int, len(matrix))
	for i := 0; i < len(matrix); i++ {
		buf[i] = make([][]int, len(matrix[0]))
	}

	max := 0
	for i := len(matrix) - 1; i >= 0; i-- {
		for j := len(matrix[0]) - 1; j >= 0; j-- {
			if matrix[i][j] == '0' {
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

			rmin := len(matrix[0])
			for n := 0; n < c+1; n++ {
				if rmin > buf[i+n][j][0] {
					rmin = buf[i+n][j][0]
				}
				if max < rmin*(n+1) {
					max = rmin * (n + 1)
				}
			}
		}
	}
	return max
}

func main() {
	fmt.Println()
}
