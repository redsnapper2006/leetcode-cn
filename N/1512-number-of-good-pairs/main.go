package main

import "fmt"

func numIdenticalPairs(nums []int) int {
	M := map[int]int{}
	for i := 0; i < len(nums); i++ {
		M[nums[i]]++
	}

	sum := 0
	for _, v := range M {
		if v > 1 {
			sum += v * (v - 1) / 2
		}
	}
	return sum
}

func main() {
	fmt.Println("a")
}
