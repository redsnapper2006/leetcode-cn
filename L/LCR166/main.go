package main

import "fmt"

func maxValue(grid [][]int) int {
	buf := make([][]int, len(grid))
	for i := 0; i < len(grid); i++ {
		buf[i] = make([]int, len(grid[0]))
	}

	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[0]); j++ {
			up, left := 0, 0
			if i > 0 {
				up = buf[i-1][j]
			}
			if j > 0 {
				left = buf[i][j-1]
			}
			sum := up
			if up < left {
				sum = left
			}
			buf[i][j] = sum + grid[i][j]
		}
	}
	return buf[len(grid)-1][len(grid[0])-1]
}

func main() {
	fmt.Println("a")
}
