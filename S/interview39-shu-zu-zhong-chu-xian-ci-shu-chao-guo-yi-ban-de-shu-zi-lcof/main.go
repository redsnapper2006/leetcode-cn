package main

import "fmt"

func majorityElement(nums []int) int {
	accum := make(map[int]int)
	for _, v := range nums {
		accum[v]++
		if accum[v] >= (len(nums)+1)/2 {
			return v
		}
	}

	return 0
}

func main() {
	fmt.Println("a")
}
