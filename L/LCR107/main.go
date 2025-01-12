package main

import "fmt"

func updateMatrix(mat [][]int) [][]int {
	buf := make([][]int, len(mat))
	for i, v := range mat {
		buf[i] = make([]int, len(v))
	}
	start := [][]int{}
	for i := 0; i < len(mat); i++ {
		for j := 0; j < len(mat[i]); j++ {
			if mat[i][j] == 0 {
				start = append(start, []int{i, j})
			}
		}
	}

	for len(start) > 0 {
		t := [][]int{}
		for _, n := range start {
			mat[n[0]][n[1]] = 0
			for _, cord := range [][]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}} {
				nr, nc := n[0]+cord[0], n[1]+cord[1]
				if nr < 0 || nr > len(mat) || nc < 0 || nc > len(mat[0]) || mat[nr][nc] == 0 {
					continue
				}
				if buf[nr][nc] == 0 || buf[nr][nc] > buf[n[0]][n[1]]+1 {
					buf[nr][nc] = buf[n[0]][n[1]] + 1
				}
				t = append(t, []int{nr, nc})
			}
		}
		start = t
	}
	return buf
}

func main() {
	fmt.Println()
}
