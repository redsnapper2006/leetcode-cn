package main

import (
	"fmt"
	"strconv"
)

func numIslands(grid [][]byte) int {

	var helperFunc func(grid [][]byte, x, y int) int
	helperFunc = func(grid [][]byte, x, y int) int {
		if grid[x][y]&1 == 0 {
			return 0
		}
		grid[x][y] = 0
		if x < len(grid)-1 && grid[x+1][y]&1 == 1 {
			helperFunc(grid, x+1, y)
		}
		if x > 0 && grid[x-1][y]&1 == 1 {
			helperFunc(grid, x-1, y)
		}
		if y < len(grid[0])-1 && grid[x][y+1]&1 == 1 {
			helperFunc(grid, x, y+1)
		}
		if y > 0 && grid[x][y-1]&1 == 1 {
			helperFunc(grid, x, y-1)
		}

		return 1
	}

	c := 0
	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[0]); j++ {
			c += helperFunc(grid, i, j)
		}
	}
	return c
}

func numIslandsV2(grid [][]byte) int {
	islands := []map[string]int{}

	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[0]); j++ {
			if grid[i][j]&1 == 0 {
				continue
			}

			rowIslandIdx, colIslandIdx := -1, -1
			if i > 0 && grid[i-1][j]&1 != 0 {
				k := strconv.Itoa(i-1) + "-" + strconv.Itoa(j)
				for isIdx, is := range islands {
					_, ok := is[k]
					if ok {
						rowIslandIdx = isIdx
						break
					}
				}
			}
			if j > 0 && grid[i][j-1]&1 != 0 {
				k := strconv.Itoa(i) + "-" + strconv.Itoa(j-1)
				for isIdx, is := range islands {
					_, ok := is[k]
					if ok {
						colIslandIdx = isIdx
						break
					}
				}
			}
			curKey := strconv.Itoa(i) + "-" + strconv.Itoa(j)
			if colIslandIdx == -1 && rowIslandIdx == -1 {
				t := map[string]int{}
				t[curKey]++
				islands = append(islands, t)
			}
			if colIslandIdx > -1 && rowIslandIdx == -1 {
				islands[colIslandIdx][curKey]++
			}
			if colIslandIdx == -1 && rowIslandIdx > -1 {
				islands[rowIslandIdx][curKey]++
			}
			if colIslandIdx > -1 && rowIslandIdx > -1 {
				if colIslandIdx == rowIslandIdx {
					islands[rowIslandIdx][curKey]++
				} else {
					var a, b int
					if rowIslandIdx < colIslandIdx {
						a = rowIslandIdx
						b = colIslandIdx
					} else {
						a = colIslandIdx
						b = rowIslandIdx

					}

					islands[a][curKey]++
					for k, v := range islands[b] {
						islands[a][k] = v
					}
					islands = append(islands[0:b], islands[b+1:]...)
				}
			}
			// fmt.Println("final", isConnected, islands)
		}
	}
	return len(islands)
}

func main() {

	fmt.Println(numIslands([][]byte{{1, 1, 1, 1, 0},
		{1, 1, 0, 1, 0},
		{1, 1, 0, 0, 0},
		{0, 0, 0, 0, 0}}))

	fmt.Println(numIslands([][]byte{{1, 1, 0, 0, 0},
		{1, 1, 0, 0, 0},
		{0, 0, 1, 0, 0},
		{0, 0, 0, 1, 1}}))

}
