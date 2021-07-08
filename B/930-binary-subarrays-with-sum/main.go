package main

import "fmt"

func numSubarraysWithSum(A []int, S int) int {
	m := map[int]int{}
	sum := 0
	ret := 0
	for _, v := range A {
		sum += v
		if sum-S == 0 {
			ret++
		}
		ret += m[sum-S]
		m[sum]++
	}
	return ret
}

func main() {
	fmt.Println()
}
