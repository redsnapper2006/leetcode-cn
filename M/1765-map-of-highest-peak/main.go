package main

import "fmt"

func highestPeak(isWater [][]int) [][]int {
	candi := [][]int{}
	for r, row := range isWater {
		for c, v := range row {
			if v == 1 {
				candi = append(candi, []int{r, c})
			}
		}
	}

	alt := 1
	points := [][]int{{-1, 0}, {1, 0}, {0, 1}, {0, -1}}
	for len(candi) > 0 {
		alt++
		t := [][]int{}

		for _, can := range candi {
			r, c := can[0], can[1]
			for _, p := range points {
				nr, nc := r+p[0], c+p[1]
				if nr >= 0 && nr < len(isWater) && nc >= 0 && nc < len(isWater[0]) && isWater[nr][nc] == 0 {
					t = append(t, []int{nr, nc})
					isWater[nr][nc] = alt
				}
			}
		}
		candi = t
	}
	for r := 0; r < len(isWater); r++ {
		for c := 0; c < len(isWater[0]); c++ {
			isWater[r][c]--
		}
	}

	return isWater
}

func main() {
	fmt.Println()
}
