package main

import (
	"fmt"
)

func largestIsland(grid [][]int) int {
	buf := make([][]int, len(grid))
	for i, v := range grid {
		buf[i] = make([]int, len(v))
		copy(buf[i], grid[i])
	}
	colors := make([][]int, len(grid))
	for i, v := range grid {
		colors[i] = make([]int, len(v))
	}
	max := -1
	cords := [][]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

	var dfs func(buf [][]int, colors [][]int, cnt *int, color, r, c int)
	dfs = func(buf [][]int, colors [][]int, cnt *int, color, r, c int) {
		// fmt.Println(color)
		*cnt++
		buf[r][c] = 0
		for _, cord := range cords {
			nr, nc := r+cord[0], c+cord[1]
			if nr >= 0 && nr < len(buf) && nc >= 0 && nc < len(buf[0]) && buf[nr][nc] == 1 {
				dfs(buf, colors, cnt, color, nr, nc)
			}
		}
		colors[r][c] = color
		if *cnt > max {
			max = *cnt
		}
	}

	colorMap := map[int]int{}
	colorcnt := 1
	for i := 0; i < len(buf); i++ {
		for j := 0; j < len(buf[0]); j++ {
			if buf[i][j] == 1 {
				cnt := 0
				dfs(buf, colors, &cnt, colorcnt, i, j)
				colorMap[colorcnt] = cnt
				colorcnt++
			}
		}
	}

	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[0]); j++ {
			if grid[i][j] == 0 {
				m := map[int]int{}
				for _, cord := range cords {
					nr, nc := i+cord[0], j+cord[1]
					if nr >= 0 && nr < len(buf) && nc >= 0 && nc < len(buf[0]) && colors[nr][nc] > 0 {
						m[colors[nr][nc]] = colorMap[colors[nr][nc]]
					}
				}
				s := 1
				for _, v := range m {
					s += v
				}
				if s > max {
					max = s
				}
			}
		}
	}
	return max
}

func main() {
	fmt.Println()
}
