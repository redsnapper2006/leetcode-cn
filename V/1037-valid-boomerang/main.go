package main

import "fmt"

func isBoomerang(points [][]int) bool {

	var k1, k2 float64

	if (points[0][0] == points[1][0] && points[0][1] == points[1][1]) || (points[1][0] == points[2][0] && points[1][1] == points[2][1]) || (points[0][0] == points[2][0] && points[0][1] == points[2][1]) {
		return false
	}

	if (points[0][0] == points[1][0] && points[1][0] == points[2][0]) || (points[0][1] == points[1][1] && points[1][1] == points[2][1]) {
		return false
	}
	if (points[0][0] == points[1][0] && points[1][0] != points[2][0]) || points[0][0] != points[1][0] && points[1][0] == points[2][0] {
		return true
	}

	if (points[0][1] == points[1][1] && points[1][1] != points[2][1]) || points[0][0] != points[1][1] && points[1][0] == points[2][1] {
		return true
	}

	k1 = float64(points[0][1]-points[1][1]) / float64(points[0][0]-points[1][0])
	k2 = float64(points[1][1]-points[2][1]) / float64(points[1][0]-points[2][0])

	if k1 == k2 {
		return false
	}
	return true
}

func main() {
	fmt.Println("a")
}
