package main

import (
	"fmt"
)

func islandPerimeter(grid [][]int) int {
	sum := 0

	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[0]); j++ {
			if grid[i][j] == 0 {
				continue
			}

			c := 4
			if i > 0 && grid[i-1][j] == 1 {
				c--
			}
			if j > 0 && grid[i][j-1] == 1 {
				c--
			}
			if i < len(grid)-1 && grid[i+1][j] == 1 {
				c--
			}
			if j < len(grid[0])-1 && grid[i][j+1] == 1 {
				c--
			}
			sum += c
		}
	}
	return sum
}

func main() {
	fmt.Println("a")
}
