package main

import (
	"fmt"
)

func canMeasureWater(x int, y int, z int) bool {
	if z == 0 {
		return true
	}
	if x == 0 || y == 0 {
		return z == x || z == y
	}
	var GCD func(x, y int) int
	GCD = func(x, y int) int {
		p1, p2 := x, y
		if p2 > p1 {
			p1, p2 = y, x
		}
		if p1%p2 == 0 {
			return p2
		}
		return GCD(p1%p2, p2)
	}

	g := GCD(x, y)

	return z%g == 0 && z >= 0 && z <= (x+y)
}

func main() {
	fmt.Println(canMeasureWater(12, 7, 3))
}
