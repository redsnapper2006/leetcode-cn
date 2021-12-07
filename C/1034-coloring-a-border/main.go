package main

import "fmt"

func colorBorder(grid [][]int, row int, col int, color int) [][]int {
	if grid[row][col] == color {
		return grid
	}
	ret := make([][]int, len(grid))
	for i, v := range grid {
		ret[i] = make([]int, len(v))
		copy(ret[i], v)
	}

	orgcolor := grid[row][col]
	candi := [][]int{{row, col}}
	for len(candi) > 0 {
		t := [][]int{}
		for _, c := range candi {
			cr, cc := c[0], c[1]
			cnt := 0
			for _, neighor := range [][]int{{0, -1}, {-1, 0}, {0, 1}, {1, 0}} {
				nr, nc := cr+neighor[0], cc+neighor[1]
				if nr >= 0 && nr < len(grid) && nc >= 0 && nc < len(grid[0]) && grid[nr][nc] == orgcolor {
					cnt++
					if ret[nr][nc] == orgcolor {
						t = append(t, []int{nr, nc})
					}
				}
			}
			if cnt < 4 {
				ret[cr][cc] = color
			} else {
				ret[cr][cc] = -1
			}
		}
		candi = t
	}
	for _, r := range ret {
		for i := 0; i < len(r); i++ {
			if r[i] == -1 {
				r[i] = orgcolor
			}
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
