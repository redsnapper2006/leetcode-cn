package main

import "fmt"

func mySqrt(x int) int {
	if x <= 1 {
		return x
	}

	s, e := 1, x-1
	for s <= e {
		m := s + (e-s)/2
		if m*m < x {
			s = m + 1
		} else if m*m > x {
			e = m - 1
		} else {
			e = m
			break
		}
	}
	return e
}

func main() {
	fmt.Println()
}
