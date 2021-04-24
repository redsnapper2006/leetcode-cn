package main

import (
	"fmt"
	"sort"
)

func purchasePlans(nums []int, target int) int {
	sort.Ints(nums)
	cnt := 0
	for i := 0; i < len(nums); i++ {
		if nums[i] >= target {
			break
		}
		s, e := i+1, len(nums)-1
		for s <= e {
			m := s + (e-s)/2
			// fmt.Println(nums[i], nums[m], nums[i]+nums[m], cnt)
			if nums[i]+nums[m] <= target {
				s = m + 1
			} else {
				e = m - 1
			}
		}
		cnt += s - i - 1
		cnt %= 1000000007
	}
	return cnt
}

func main() {
	fmt.Println(purchasePlans([]int{2, 5, 3, 5}, 6))
}
