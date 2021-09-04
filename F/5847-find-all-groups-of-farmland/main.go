package main

import "fmt"

func findFarmland(land [][]int) [][]int {
	ret := [][]int{}

	for i := 0; i < len(land); i++ {
		for j := 0; j < len(land[0]); j++ {
			if land[i][j] == 0 {
				continue
			}
			r := i
			for r < len(land) && land[r][j] == 1 {
				r++
			}
			r--
			c := j
			for c < len(land[0]) && land[i][c] == 1 {
				c++
			}
			c--
			for m := i; m <= r; m++ {
				for n := j; n <= c; n++ {
					land[m][n] = 0
				}
			}
			ret = append(ret, []int{i, j, r, c})
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
