package main

import (
	"fmt"
	"sort"
)

func smallerNumbersThanCurrent(nums []int) []int {
	if len(nums) == 0 {
		return []int{}
	}
	if len(nums) == 1 {
		return []int{0}
	}

	buf := make([]int, len(nums))
	copy(buf, nums)

	sort.Ints(buf)
	m := make(map[int]int)
	m[buf[0]] = 0
	for i := 1; i < len(buf); i++ {
		if buf[i] > buf[i-1] {
			m[buf[i]] = i
		}
	}
	for i := 0; i < len(nums); i++ {
		buf[i] = m[nums[i]]
	}
	return buf
}

func main() {
	fmt.Println("a")
}
