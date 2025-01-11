package main

import "fmt"

func twoSum(nums []int, target int) []int {

	s, e := 0, len(nums)-1
	for s <= e {
		if nums[s]+nums[e] == target {
			return []int{nums[s], nums[e]}
		} else if nums[s]+nums[e] > target {
			e--
		} else {
			s++
		}
	}

	return nil
}

func main() {
	fmt.Println("a")
}
