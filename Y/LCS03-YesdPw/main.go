package main

import "fmt"

func largestArea(grid []string) int {
	buf := make([][]int, len(grid))
	for i := 0; i < len(grid); i++ {
		buf[i] = make([]int, len(grid[i]))
		for j := 0; j < len(grid[i]); j++ {
			buf[i][j] = int(grid[i][j] - '0')
			if buf[i][j] == 0 {
				buf[i][j] = -1
			}
		}
	}

	var dfs func(buf [][]int, x, y int, color int)
	dfs = func(buf [][]int, x, y int, color int) {
		buf[x][y] = 0
		cord := [][]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}
		for i := 0; i < len(cord); i++ {
			newx, newy := x+cord[i][0], y+cord[i][1]
			if newx >= 0 && newx < len(buf) && newy >= 0 && newy < len(buf[0]) && buf[newx][newy] == color {
				dfs(buf, newx, newy, color)
			}
		}
	}

	for m := 0; m < len(buf); m++ {
		for n := 0; n < len(buf[0]); n++ {
			if buf[m][n] == -1 {
				cord := [][]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}
				for i := 0; i < len(cord); i++ {
					newx, newy := m+cord[i][0], n+cord[i][1]
					if newx >= 0 && newx < len(buf) && newy >= 0 && newy < len(buf[0]) && buf[newx][newy] > 0 {
						dfs(buf, newx, newy, buf[newx][newy])
					}
				}
			}
		}
	}

	for i := 0; i < len(buf[0]); i++ {
		if buf[0][i] <= 0 {
			continue
		}
		dfs(buf, 0, i, buf[0][i])
	}
	for i := 0; i < len(buf[0]); i++ {
		if buf[len(buf)-1][i] <= 0 {
			continue
		}
		dfs(buf, len(buf)-1, i, buf[len(buf)-1][i])
	}
	for j := 0; j < len(buf); j++ {
		if buf[j][0] <= 0 {
			continue
		}
		dfs(buf, j, 0, buf[j][0])
	}
	for j := 0; j < len(buf); j++ {
		if buf[j][len(buf[0])-1] <= 0 {
			continue
		}
		dfs(buf, j, len(buf[0])-1, buf[j][len(buf[0])-1])
	}
	// fmt.Println(buf)
	max := 0
	var dfs2 func(buf [][]int, x, y, color int, count *int)
	dfs2 = func(buf [][]int, x, y, color int, count *int) {
		buf[x][y] = 0
		cord := [][]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}
		isFound := false
		for i := 0; i < len(cord); i++ {
			newx, newy := x+cord[i][0], y+cord[i][1]
			if newx >= 0 && newx < len(buf) && newy >= 0 && newy < len(buf[0]) && buf[newx][newy] == color {
				isFound = true
				// buf[newx][newy] = 0
				*count++
				dfs2(buf, newx, newy, color, count)
			}
		}
		if !isFound && *count > max {
			max = *count
		}
	}
	for i := 0; i < len(buf); i++ {
		for j := 0; j < len(buf[0]); j++ {
			if buf[i][j] <= 0 {
				continue
			}
			count := 1
			dfs2(buf, i, j, buf[i][j], &count)
		}
	}

	return max
}

func main() {
	fmt.Println(largestArea([]string{"110", "231", "221"}))
	fmt.Println(largestArea([]string{"11111100000", "21243101111", "21224101221", "11111101111"}))
	fmt.Println(largestArea([]string{"000", "010", "000"})) // ["000","010","000"]
}
