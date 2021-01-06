package main

import "fmt"

func leastBricks(wall [][]int) int {
	total := 0
	for i := 0; i < len(wall[0]); i++ {
		total += wall[0][i]
	}
	max := 0

	buf := map[int]int{}
	for i := 0; i < len(wall); i++ {
		start := 0
		for j := 0; j < len(wall[i]); j++ {
			buf[start+wall[i][j]]++
			start += wall[i][j]
		}
	}
	for k, v := range buf {
		if k != total && v > max {
			max = v
		}
	}
	return len(wall) - max
}

func main() {
	fmt.Println("a")
}
