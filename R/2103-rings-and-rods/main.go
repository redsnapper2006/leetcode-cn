package main

import "fmt"

func countPoints(rings string) int {
	buf := make([][]int, 10)
	for i := 0; i < 10; i++ {
		buf[i] = make([]int, 3)
	}
	for i := 0; i < len(rings); i = i + 2 {
		idx := -1
		if byte(rings[i]) == byte('R') {
			idx = 0
		} else if byte(rings[i]) == byte('G') {
			idx = 1
		} else if byte(rings[i]) == byte('B') {
			idx = 2
		}
		offset := int(byte(rings[i+1]) - byte('0'))
		buf[offset][idx]++
	}
	cnt := 0
	for i := 0; i < 10; i++ {
		if buf[i][0] > 0 && buf[i][1] > 0 && buf[i][2] > 0 {
			cnt++
		}
	}
	return cnt
}

func main() {
	fmt.Println()
}
