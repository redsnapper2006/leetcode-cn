package main

import (
	"fmt"
)

func numberOfBoomerangs(points [][]int) int {
	accum := 0
	for i := 0; i < len(points); i++ {
		p1 := points[i]
		buf := make(map[int]int)
		for j := 0; j < len(points); j++ {
			p2 := points[j]
			distance := (p1[0]-p2[0])*(p1[0]-p2[0]) + (p1[1]-p2[1])*(p1[1]-p2[1])
			buf[distance]++
		}
		for _, c := range buf {
			accum += c * (c - 1)
		}
	}

	return accum
}

func main() {
	fmt.Println(numberOfBoomerangs([][]int{{0, 0}, {1, 0}, {2, 0}}))
}
