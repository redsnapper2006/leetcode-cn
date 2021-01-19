package main

import "fmt"

func maxDistToClosest(seats []int) int {
	var buf []int
	for i, b := range seats {
		if b == 0 {
			continue
		}
		buf = append(buf, i)
	}
	max := -1
	if buf[0] != 0 {
		max = buf[0]
	}
	for i := 1; i < len(buf); i++ {
		t := (buf[i] - buf[i-1]) / 2
		if t > max {
			max = t
		}
	}
	if buf[len(buf)-1] != len(seats)-1 {
		if len(seats)-1-buf[len(buf)-1] > max {
			max = len(seats) - 1 - buf[len(buf)-1]
		}
	}
	return max
}

func main() {
	fmt.Println()
}
