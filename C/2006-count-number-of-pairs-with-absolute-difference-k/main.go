package main

import "fmt"

func countKDifference(nums []int, k int) int {
	m := map[int]int{}
	ret := 0
	for _, n := range nums {
		ret += m[n-k]
		if k != 0 {
			ret += m[n+k]
		}
		m[n]++
	}
	return ret
}

func main() {
	fmt.Println()
}
