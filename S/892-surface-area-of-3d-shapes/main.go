package main

import "fmt"

func surfaceArea(grid [][]int) int {
	MIN := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}

	sum := 0
	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[0]); j++ {
			c := grid[i][j]
			if c == 0 {
				continue
			}
			sum += c*6 - (c-1)*2
			if i > 0 {
				sum -= MIN(c, grid[i-1][j])
			}
			if j > 0 {
				sum -= MIN(c, grid[i][j-1])
			}
			if i < len(grid)-1 {
				sum -= MIN(c, grid[i+1][j])
			}
			if j < len(grid[0])-1 {
				sum -= MIN(c, grid[i][j+1])
			}
		}
	}
	return sum
}

func main() {
	fmt.Println(surfaceArea([][]int{[]int{2, 2, 2}, []int{2, 1, 2}, []int{2, 2, 2}}))
}
