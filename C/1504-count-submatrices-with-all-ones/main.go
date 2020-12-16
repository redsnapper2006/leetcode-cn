package main

import "fmt"

func numSubmat(mat [][]int) int {
	buf := make([][][]int, len(mat))
	for i := 0; i < len(mat); i++ {
		buf[i] = make([][]int, len(mat[0]))
	}

	sum := 0
	for i := len(mat) - 1; i >= 0; i-- {
		for j := len(mat[0]) - 1; j >= 0; j-- {
			if mat[i][j] == 0 {
				buf[i][j] = []int{0, 0}
				continue
			}
			r, c := 0, 0
			if i < len(mat)-1 {
				c = buf[i+1][j][1]
			}
			if j < len(mat[0])-1 {
				r = buf[i][j+1][0]
			}
			buf[i][j] = []int{r + 1, c + 1}

			min := len(mat[0])
			for m := 0; m < c+1; m++ {
				if min > buf[i+m][j][0] {
					min = buf[i+m][j][0]
				}
				sum += min
			}
		}
	}
	return sum
}

func main() {
	fmt.Println()
}
