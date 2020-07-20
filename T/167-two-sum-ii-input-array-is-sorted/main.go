package main

import "fmt"

func twoSumV2(nums []int, target int) []int {
	m := make(map[int]int)
	for i := 0; i < len(nums); i++ {
		v, ok := m[target-nums[i]]
		if ok {
			return []int{v + 1, i + 1}
		} else {
			m[nums[i]] = i
		}
	}
	return []int{}
}

func twoSum(nums []int, target int) []int {
	s, e := 0, len(nums)-1
	for s < e {
		if nums[s]+nums[e] == target {
			return []int{s + 1, e + 1}
		} else if nums[s]+nums[e] < target {
			s++
		} else {
			e--
		}
	}
	return []int{}
}

func main() {
	fmt.Println("a")
}
