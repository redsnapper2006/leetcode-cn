package main

import "fmt"

func findMaxLength(nums []int) int {
	m := map[int]int{}
	zc, oc := 0, 0
	m[0] = -1
	max := 0
	for i, n := range nums {
		if n == 0 {
			zc++
		} else {
			oc++
		}
		v, ok := m[zc-oc]
		if !ok {
			m[zc-oc] = i
		} else if i-v > max {
			max = i - v
		}
	}
	return max
}

func main() {
	fmt.Println()
}
