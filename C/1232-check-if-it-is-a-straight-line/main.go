package main

import "fmt"

func checkStraightLine(coordinates [][]int) bool {
	if len(coordinates) == 2 {
		return true
	}
	a, b := float64(0.0), float64(0.0)
	isHor, isVer := false, false
	x, y := 0, 0
	p1, p2 := coordinates[0], coordinates[1]
	if p1[0] == p2[0] {
		isVer = true
		x = p1[0]
	} else if p1[1] == p2[1] {
		isHor = true
		y = p1[1]
	} else {
		a = float64(p1[1]-p2[1]) / float64(p1[0]-p2[0])
		b = float64(p1[1]) - a*float64(p1[0])
	}
	for i := 2; i < len(coordinates); i++ {
		candi := coordinates[i]
		if isVer {
			if candi[0] != x {
				return false
			}
		} else if isHor {
			if candi[1] != y {
				return false
			}
		} else {
			if float64(candi[1]) != a*float64(candi[0])+b {
				return false
			}
		}
	}
	return true
}

func main() {
	fmt.Println("a")
}
