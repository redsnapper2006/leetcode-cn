package main

import (
	"fmt"
	"sort"
)

func nextPermutation(nums []int) {
	if len(nums) <= 1 {
		return
	}
	base := nums[len(nums)-1]
	idx := len(nums) - 1
	for i := len(nums) - 2; i >= 0; i-- {
		if base > nums[i] {
			idx = i
			break
		} else {
			base = nums[i]
		}
	}
	if idx == len(nums)-1 {
		s, e := 0, len(nums)-1
		for s < e {
			nums[s], nums[e] = nums[e], nums[s]
			s++
			e--
		}
		return
	}
	s, e := idx+1, len(nums)-1
	eIdx := -1
	for s < e {
		m := s + (e-s)/2
		if nums[m] > nums[idx] {
			s = m + 1
		} else if nums[m] < nums[idx] {
			e = m - 1
		} else {
			eIdx = m
			break
		}
	}
	var replace int
	if eIdx != -1 || nums[s] <= nums[idx] {
		b := s
		if eIdx != -1 {
			b = eIdx
		}
		for i := b; i > idx; i-- {
			if nums[i] > nums[idx] {
				replace = i
				break
			}
		}
	} else {
		replace = s
	}

	nums[idx], nums[replace] = nums[replace], nums[idx]
	s, e := idx+1, len(nums)-1
	for s < e {
		nums[s], nums[e] = nums[e], nums[s]
		s++
		e--
	}
	return
}

func main() {
	a := []int{3, 2, 1}
	nextPermutation(a)
	fmt.Println(a)

	a = []int{2, 3, 2, 1}
	nextPermutation(a)
	fmt.Println(a)
}
