package main

import "fmt"

func shortestPathBinaryMatrix(grid [][]int) int {
	if grid[0][0] != 0 {
		return -1
	}
	buf := make([][]int, len(grid))
	for i := 0; i < len(grid); i++ {
		buf[i] = make([]int, len(grid[0]))
	}
	buf[0][0] = 1
	stack := [][]int{{0, 0}}
	for len(stack) > 0 {
		var t [][]int
		for i := 0; i < len(stack); i++ {
			r, c := stack[i][0], stack[i][1]
			if r > 0 {
				if c > 1 && grid[r-1][c-1] == 0 && buf[r-1][c-1] == 0 {
					t = append(t, []int{r - 1, c - 1})
					buf[r-1][c-1] = buf[r][c] + 1
				}
				if grid[r-1][c] == 0 && buf[r-1][c] == 0 {
					t = append(t, []int{r - 1, c})
					buf[r-1][c] = buf[r][c] + 1
				}
				if c < len(grid[0])-1 && grid[r-1][c+1] == 0 && buf[r-1][c+1] == 0 {
					t = append(t, []int{r - 1, c + 1})
					buf[r-1][c+1] = buf[r][c] + 1
				}
			}
			if c > 1 && grid[r][c-1] == 0 && buf[r][c-1] == 0 {
				t = append(t, []int{r, c - 1})
				buf[r][c-1] = buf[r][c] + 1
			}
			if c < len(grid[0])-1 && grid[r][c+1] == 0 && buf[r][c+1] == 0 {
				t = append(t, []int{r, c + 1})
				buf[r][c+1] = buf[r][c] + 1
			}
			if r < len(grid)-1 {
				if c > 1 && grid[r+1][c-1] == 0 && buf[r+1][c-1] == 0 {
					t = append(t, []int{r + 1, c - 1})
					buf[r+1][c-1] = buf[r][c] + 1
				}
				if grid[r+1][c] == 0 && buf[r+1][c] == 0 {
					t = append(t, []int{r + 1, c})
					buf[r+1][c] = buf[r][c] + 1
				}
				if c < len(grid[0])-1 && grid[r+1][c+1] == 0 && buf[r+1][c+1] == 0 {
					t = append(t, []int{r + 1, c + 1})
					buf[r+1][c+1] = buf[r][c] + 1
				}
			}
		}
		stack = t
	}

	if buf[len(grid)-1][len(grid[0])-1] == 0 {
		return -1
	}
	return buf[len(grid)-1][len(grid[0])-1]
}

func main() {

	fmt.Println(shortestPathBinaryMatrix([][]int{{0, 0, 1, 0, 0, 0, 0}, {0, 1, 0, 0, 0, 0, 1}, {0, 0, 1, 0, 1, 0, 0}, {0, 0, 0, 1, 1, 1, 0}, {1, 0, 0, 1, 1, 0, 0}, {1, 1, 1, 1, 1, 0, 1}, {0, 0, 1, 0, 0, 0, 0}}))
}
