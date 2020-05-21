package main

import "fmt"

func singleNumber(nums []int) []int {
	sum := 0
	for i := 0; i < len(nums); i++ {
		sum ^= nums[i]
	}

	flag := (-sum) & sum
	a, b := 0, 0
	for i := 0; i < len(nums); i++ {
		if flag&nums[i] == 0 {
			a ^= nums[i]
		} else {
			b ^= nums[i]
		}
	}
	return []int{a, b}
}

func main() {
	fmt.Println("a")
}
