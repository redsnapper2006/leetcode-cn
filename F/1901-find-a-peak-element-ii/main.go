package main

import "fmt"

func findPeakGrid(mat [][]int) []int {
	visit := make([][]int, len(mat))
	for i := 0; i < len(mat); i++ {
		visit[i] = make([]int, len(mat[0]))
	}
	var dfs func(mat, buf [][]int, x, y int) (int, int, bool)
	dfs = func(mat, buf [][]int, x, y int) (int, int, bool) {
		if buf[x][y] == 1 {
			return x, y, false
		}
		buf[x][y] = 1
		isTarget := true
		pos := [][]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}
		for i := 0; i < len(pos); i++ {
			r, c := x+pos[i][0], y+pos[i][1]
			if r >= 0 && r < len(mat) && c >= 0 && c < len(mat[0]) && mat[r][c] >= mat[x][y] {
				isTarget = false
				break
			}
		}
		if isTarget {
			return x, y, true
		}
		for i := 0; i < len(pos); i++ {
			r, c := x+pos[i][0], y+pos[i][1]
			if r >= 0 && r < len(mat) && c >= 0 && c < len(mat[0]) && mat[r][c] >= mat[x][y] && buf[r][c] == 0 {
				newr, newc, ret := dfs(mat, buf, r, c)
				if ret {
					return newr, newc, ret
				}
			}
		}
		return x, y, false
	}
	for i := 0; i < len(mat); i++ {
		for j := 0; j < len(mat[0]); j++ {
			if visit[i][j] == 0 {
				r, c, ret := dfs(mat, visit, i, j)
				if ret {
					return []int{r, c}
				}
			}
		}
	}
	return []int{-1, -1}
}

func main() {
	fmt.Println()
}
