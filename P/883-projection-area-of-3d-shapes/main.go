package main

import "fmt"

func projectionArea(grid [][]int) int {
	xcnt, ycnt, zcnt := 0, 0, 0
	for _, v := range grid {
		xmax := -1
		for _, c := range v {
			if c > 0 {
				zcnt++
			}
			if c > xmax {
				xmax = c
			}
		}
		xcnt += xmax
	}
	for i := 0; i < len(grid[0]); i++ {
		ymax := -1
		for j := 0; j < len(grid); j++ {
			if grid[j][i] > ymax {
				ymax = grid[j][i]
			}
		}
		ycnt += ymax
	}
	return xcnt + ycnt + zcnt
}

func main() {
	fmt.Println("a")
}
