package main

import "fmt"

func orangesRotting(grid [][]int) int {
	if len(grid) == 0 || len(grid[0]) == 0 {
		return 0
	}
	filter := func(grid [][]int) ([][]int, bool) {
		var pos [][]int
		fresh := false
		for i := 0; i < len(grid); i++ {
			for j := 0; j < len(grid[0]); j++ {
				if grid[i][j] == 2 {
					pos = append(pos, []int{i, j})
				}
				if grid[i][j] == 1 {
					fresh = true
				}
			}
		}
		return pos, fresh
	}
	mark := func(grid [][]int, candi [][]int) {
		for i := 0; i < len(candi); i++ {
			pos := candi[i]
			grid[pos[0]][pos[1]] = 0
			if pos[0] > 0 && grid[pos[0]-1][pos[1]] == 1 {
				grid[pos[0]-1][pos[1]] = 2
			}
			if pos[0] < len(grid)-1 && grid[pos[0]+1][pos[1]] == 1 {
				grid[pos[0]+1][pos[1]] = 2
			}
			if pos[1] > 0 && grid[pos[0]][pos[1]-1] == 1 {
				grid[pos[0]][pos[1]-1] = 2
			}
			if pos[1] < len(grid[0])-1 && grid[pos[0]][pos[1]+1] == 1 {
				grid[pos[0]][pos[1]+1] = 2
			}
		}
	}

	candi, fresh := filter(grid)
	if len(candi) == 0 && !fresh {
		return 0
	}
	if len(candi) == 0 && fresh {
		return -1
	}
	if len(candi) > 0 && !fresh {
		return 0
	}

	times := 0
	for {
		mark(grid, candi)
		times++
		candi, fresh = filter(grid)
		if len(candi) == 0 && !fresh {
			return times - 1
		}
		if len(candi) == 0 && fresh {
			return -1
		}
		if len(candi) > 0 && !fresh {
			return times
		}
	}
}

func main() {
	fmt.Println(orangesRotting([][]int{{2, 1, 1}, {1, 1, 0}, {0, 1, 1}}))
}
