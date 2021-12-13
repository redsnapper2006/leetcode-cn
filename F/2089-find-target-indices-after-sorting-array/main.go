package main

import (
	"fmt"
	"sort"
)

func targetIndices(nums []int, target int) []int {
	sort.Ints(nums)
	s, e := 0, len(nums)-1
	for s <= e {
		m := s + (e-s)/2
		if nums[m] > target {
			e = m - 1
		} else if nums[m] <= target {
			s = m + 1
		}
	}
	s--
	if s < 0 || nums[s] != target {
		return []int{}
	}
	end := s
	for end >= 0 && nums[end] == target {
		end--
	}
	if end+1 == s {
		return []int{end + 1}
	}
	ret := []int{}
	for i := end + 1; i <= s; i++ {
		ret = append(ret, i)
	}
	return ret
}

func main() {
	fmt.Println()
}
