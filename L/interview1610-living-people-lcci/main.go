package main

import "fmt"

func maxAliveYear(birth []int, death []int) int {
	buf := make([]int, 101)
	for i := 0; i < len(birth); i++ {
		s := birth[i] - 1900
		e := death[i] - 1900
		for j := s; j <= e; j++ {
			buf[j]++
		}
	}
	max := -1
	maxIdx := -1
	for i := 0; i < len(buf); i++ {
		if buf[i] > max {
			max = buf[i]
			maxIdx = i
		}
	}
	return maxIdx + 1900
}

func main() {
	fmt.Println("a")
}
