package main

import "fmt"

func nearestValidPoint(x int, y int, points [][]int) int {
	distance := 1<<31 - 1
	idx := -1
	for i, p := range points {
		if p[0] == x || p[1] == y {
			dx := p[0] - x
			if dx < 0 {
				dx = -dx
			}
			dy := p[1] - y
			if dy < 0 {
				dy = -dy
			}
			if dx+dy < distance {
				distance = dx + dy
				idx = i
			}
		}
	}
	return idx
}

func main() {
	fmt.Println("")
}
