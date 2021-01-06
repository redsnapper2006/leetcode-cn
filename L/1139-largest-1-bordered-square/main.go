package main

import "fmt"

func largest1BorderedSquare(grid [][]int) int {
	buf := make([][][]int, len(grid))
	for i := 0; i < len(grid); i++ {
		buf[i] = make([][]int, len(grid[0]))
	}
	max := -1
	for i := len(grid) - 1; i >= 0; i-- {
		for j := len(grid[0]) - 1; j >= 0; j-- {
			if grid[i][j] == 0 {
				buf[i][j] = []int{0, 0}
				continue
			}

			r, c := 1, 1
			if j < len(grid[0])-1 {
				r = buf[i][j+1][0] + 1
			}
			if i < len(grid)-1 {
				c = buf[i+1][j][1] + 1
			}
			buf[i][j] = []int{r, c}
			cnt := r
			if cnt > c {
				cnt = c
			}
			for s := cnt - 1; s >= 0; s-- {
				if buf[i][j+s][1] >= s+1 && buf[i+s][j][0] >= s+1 && s > max {
					max = s
				}
			}
		}
	}

	if max == -1 {
		return 0
	}
	return (max + 1) * (max + 1)
}

func main() {
	fmt.Println()
}
