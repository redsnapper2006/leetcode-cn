package main

import (
	"fmt"
	"math"
)

func constructRectangle(area int) []int {
	for i := int(math.Sqrt(float64(area))) + 1; i >= 1; i-- {
		if area%i == 0 {
			d1, d2 := i, area/i
			if d1 < d2 {
				d1, d2 = d2, d1
			}
			return []int{d1, d2}
		}
	}
	return nil
}

func main() {
	fmt.Println(constructRectangle(3))
}
