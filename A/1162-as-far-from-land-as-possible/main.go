package main

import "fmt"

func maxDistance(grid [][]int) int {
	var buf [][]int

	step := 1
	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[0]); j++ {
			if grid[i][j] == step {
				buf = append(buf, []int{i, j})
			}
		}
	}
	if len(buf) == 0 || len(buf) == len(grid)*len(grid[0]) {
		return -1
	}

	for len(buf) > 0 {
		step++
		var t [][]int
		for i := 0; i < len(buf); i++ {
			r, c := buf[i][0], buf[i][1]
			if r > 0 && grid[r-1][c] == 0 {
				grid[r-1][c] = step
				t = append(t, []int{r - 1, c})
			}
			if c > 0 && grid[r][c-1] == 0 {
				grid[r][c-1] = step
				t = append(t, []int{r, c - 1})
			}
			if r < len(grid)-1 && grid[r+1][c] == 0 {
				grid[r+1][c] = step
				t = append(t, []int{r + 1, c})
			}
			if c < len(grid[0])-1 && grid[r][c+1] == 0 {
				grid[r][c+1] = step
				t = append(t, []int{r, c + 1})
			}
		}
		buf = t
	}
	return step - 2
}

func main() {
	fmt.Println("a")
}
