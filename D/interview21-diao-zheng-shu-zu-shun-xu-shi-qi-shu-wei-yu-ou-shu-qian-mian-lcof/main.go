package main

import "fmt"

func exchange(nums []int) []int {
	s, e := 0, len(nums)-1
	for s < e {
		for s < e {
			if nums[s]%2 == 0 {
				break
			}
			s++
		}

		for e > s {
			if nums[e]%2 == 1 {
				break
			}
			e--
		}
		nums[s], nums[e] = nums[e], nums[s]
	}
	return nums
}

func main() {
	fmt.Println("a")
}
