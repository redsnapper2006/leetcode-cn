package main

import (
	"fmt"
)

func findRadius(houses []int, heaters []int) int {
	r := -1
	for i := 0; i < len(houses); i++ {
		b := houses[i]
		min := 1<<31 - 1
		for j := 0; j < len(heaters); j++ {
			dis := heaters[j] - b
			if dis < 0 {
				dis = -dis
			}
			if dis < min {
				min = dis
			}
		}
		if min > r {
			r = min
		}
	}
	return r
}

func main() {
	fmt.Println("a")
}
