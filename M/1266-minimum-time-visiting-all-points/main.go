package main

import "fmt"

func minTimeToVisitAllPoints(points [][]int) int {
	if len(points) <= 1 {
		return 0
	}
	sum := 0
	x1, y1 := points[0][0], points[0][1]
	for i := 1; i < len(points); i++ {
		x2, y2 := points[i][0], points[i][1]
		divx, divy := x1-x2, y1-y2
		if divx < 0 {
			divx = -divx
		}
		if divy < 0 {
			divy = -divy
		}
		v := divx
		if divx < divy {
			v = divy
		}
		sum += v
		x1, y1 = x2, y2
	}
	return sum
}

func main() {
	fmt.Println("a")
}
