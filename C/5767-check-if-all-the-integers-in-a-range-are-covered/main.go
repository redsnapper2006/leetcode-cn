package main

import "fmt"

func isCovered(ranges [][]int, left int, right int) bool {
	buf := make([]int, right+1)
	for i := 0; i < len(ranges); i++ {
		s, e := ranges[i][0], ranges[i][1]
		for j := s; j <= e; j++ {
			if j > right {
				continue
			}
			buf[j] = 1
		}
	}
	for i := left; i <= right; i++ {
		if buf[i] == 0 {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println()
}
