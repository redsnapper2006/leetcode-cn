package main

import "fmt"

func countNegatives(grid [][]int) int {
	r := len(grid) - 1
	c := 0

	count := 0
	for r >= 0 && c < len(grid[0]) {
		if grid[r][c] < 0 {
			count += len(grid[0]) - c
			r--
		} else {
			c++
		}
	}
	return count
}

func main() {
	fmt.Println("a")
}
