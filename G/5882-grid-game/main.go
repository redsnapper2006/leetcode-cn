package main

import "fmt"

func gridGame(grid [][]int) int64 {
	grid[0][0] = 0
	grid[1][len(grid[0])-1] = 0
	buf := make([][]int64, len(grid))
	for i := 0; i < len(grid); i++ {
		buf[i] = make([]int64, len(grid[0]))
	}
	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[0]); j++ {
			if j == 0 {
				buf[i][j] = int64(grid[i][j])
				continue
			}
			buf[i][j] = buf[i][j-1] + int64(grid[i][j])
		}
	}

	var min int64 = 1<<63 - 1
	for i := 0; i < len(grid[0]); i++ {
		v := buf[1][i] - int64(grid[1][i])
		if v < buf[0][len(buf[0])-1]-buf[0][i] {
			v = buf[0][len(buf[0])-1] - buf[0][i]
		}
		if v < min {
			min = v
		}
	}
	return min
}

func main() {
	fmt.Println(gridGame([][]int{{20, 3, 20, 17, 2, 12, 15, 17, 4, 15}, {20, 10, 13, 14, 15, 5, 2, 3, 14, 3}}))
}
