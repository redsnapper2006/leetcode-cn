package main

import (
	"fmt"
	"math"
)

func largestTriangleArea(points [][]int) float64 {
	buf := make([][]float64, len(points))
	for i := 0; i < len(points); i++ {
		buf[i] = make([]float64, len(points))
		for j := i + 1; j < len(points); j++ {
			buf[i][j] = math.Sqrt(float64((points[i][0]-points[j][0])*(points[i][0]-points[j][0]) + (points[i][1]-points[j][1])*(points[i][1]-points[j][1])))
		}
	}
	max := -1.0
	for i := 0; i < len(points); i++ {
		for j := i + 1; j < len(points); j++ {
			for m := j + 1; m < len(points); m++ {
				p := (buf[i][j] + buf[i][m] + buf[j][m]) / 2
				v := math.Sqrt(p * (p - buf[i][j]) * (p - buf[i][m]) * (p - buf[j][m]))
				if v > max {
					max = v
				}
			}
		}
	}
	return max
}

func main() {
	fmt.Println("a")
}
