package main

import "fmt"

func numSpecial(mat [][]int) int {
	row, col := make([]int, len(mat)), make([]int, len(mat[0]))
	for i := 0; i < len(mat); i++ {
		idx := -1
		for j := 0; j < len(mat[0]); j++ {
			if mat[i][j] == 1 {
				if idx == -1 {
					idx = j
				} else {
					idx = -1
					break
				}
			}
		}
		row[i] = idx
	}
	for i := 0; i < len(mat[0]); i++ {
		idx := -1
		for j := 0; j < len(mat); j++ {
			if mat[j][i] == 1 {
				if idx == -1 {
					idx = j
				} else {
					idx = -1
					break
				}
			}
		}
		col[i] = idx
	}

	ret := 0
	for i := 0; i < len(row); i++ {
		if row[i] == -1 {
			continue
		}
		if col[row[i]] != -1 {
			ret++
		}

	}
	return ret
}

func main() {
	fmt.Println("a")
}
