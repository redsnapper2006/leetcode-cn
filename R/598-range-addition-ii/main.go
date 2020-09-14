package main

import "fmt"

func maxCount(m int, n int, ops [][]int) int {
	aMin, bMin := m, n
	for _, v := range ops {
		a, b := v[0], v[1]
		if aMin > a {
			aMin = a
		}
		if bMin > b {
			bMin = b
		}
	}
	return aMin * bMin
}

func main() {
	fmt.Println("a")
}
