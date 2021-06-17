package main

import (
	"fmt"
)



func mySqrtBitWise(x int) int {
	if x < 2 {
		return x
	}
	small := mySqrt(x>>2) << 1
	large := small + 1
	if large*large > x {
		return small
	}

	return large
}

func mySqrt(x int) int {
	if x < 2 {
		return x
	}
	x0 := x
	x1 := (x0 + x/x0) / 2
	for {

		if x1*x1 <= x {
			break
		}
		x0 = x1
		x1 = (x0 + x/x0) / 2
	}

	return x1
}

func main() {
	fmt.Println("a")
}
