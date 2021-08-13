package main

import "fmt"

func subarraySum(nums []int, k int) int {
	sum := 0
	m := map[int]int{}
	m[0] = 1
	ret := 0
	for _, v := range nums {
		sum += v
		div := sum - k
		c, ok := m[div]
		if ok {
			ret += c
		}
		m[sum]++
	}
	return ret
}

func main() {
	fmt.Println()
}
