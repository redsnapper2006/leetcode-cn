package main

import "fmt"

func numMagicSquaresInside(grid [][]int) int {
	ret := 0
	for i := 0; i <= len(grid)-3; i++ {
		for j := 0; j <= len(grid[0])-3; j++ {
			buf := make([]int, 9)
			t := []int{
				grid[i][j], grid[i][j+1], grid[i][j+2],
				grid[i+1][j], grid[i+1][j+1], grid[i+1][j+2],
				grid[i+2][j], grid[i+2][j+1], grid[i+2][j+2],
			}
			isValid := true
			for _, v := range t {
				if v < 1 || v > 9 {
					isValid = false
					break
				}
				buf[v-1]++
				if buf[v-1] > 1 {
					isValid = false
					break
				}
			}
			if !isValid {
				continue
			}

			r1 := grid[i][j] + grid[i][j+1] + grid[i][j+2]
			r2 := grid[i+1][j] + grid[i+1][j+1] + grid[i+1][j+2]
			r3 := grid[i+2][j] + grid[i+2][j+1] + grid[i+2][j+2]
			c1 := grid[i][j] + grid[i+1][j] + grid[i+2][j]
			c2 := grid[i][j+1] + grid[i+1][j+1] + grid[i+2][j+1]
			c3 := grid[i][j+2] + grid[i+1][j+2] + grid[i+2][j+2]
			ss1 := grid[i][j] + grid[i+1][j+1] + grid[i+2][j+2]
			ss2 := grid[i+2][j] + grid[i+1][j+1] + grid[i][j+2]
			if r1 == r2 && r2 == r3 && r3 == c1 && c1 == c2 && c2 == c3 && c3 == ss1 && ss1 == ss2 {
				ret++
			}
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
