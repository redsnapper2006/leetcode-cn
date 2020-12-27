package main

import "fmt"

func findBall(grid [][]int) []int {
	var ret []int
	for i := 0; i < len(grid[0]); i++ {
		row := 0
		col := i

		for row < len(grid) {
			flag := grid[row][col]
			nextcol := col
			if flag == 1 {
				nextcol = col + 1
			} else {
				nextcol = col - 1
			}
			if nextcol < 0 || nextcol >= len(grid[0]) {
				ret = append(ret, -1)
				break
			}
			if grid[row][nextcol] != grid[row][col] {
				ret = append(ret, -1)
				break
			}
			row++
			if flag == 1 {
				col++
			} else {
				col--
			}
		}
		if row == len(grid) {
			ret = append(ret, col)
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
