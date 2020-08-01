package main

import (
	"fmt"
	"sort"
)

func smallestRange(nums [][]int) []int {
	var buf []int
	for i := 0; i < len(nums); i++ {
		for j := 0; j < len(nums[i]); j++ {
			buf = append(buf, nums[i][j])
		}
	}
	sort.Ints(buf)
	var candi []int
	for i := 0; i < len(buf); i++ {
		if i > 0 && buf[i] == buf[i-1] {
			continue
		}
		candi = append(candi, buf[i])
	}

	count := make([]int, len(nums))
	bs := func(nums []int, target int) bool {
		s, e := 0, len(nums)-1
		for s <= e {
			m := s + (e-s)/2
			if nums[m] == target {
				return true
			} else if nums[m] < target {
				s = m + 1
			} else {
				e = m - 1
			}
		}
		return false
	}
	isHit := func(count []int) bool {
		for i := 0; i < len(count); i++ {
			if count[i] == 0 {
				return false
			}
		}
		return true
	}
	var ret []int
	sIdx, eIdx := 0, -1
	minInterval := 1<<31 - 1
	for i := 0; i < len(candi); i++ {
		for j := 0; j < len(nums); j++ {
			if bs(nums[j], candi[i]) {
				count[j]++
			}
		}
		if isHit(count) {
			eIdx = i
			t := -1
			for m := sIdx; m <= eIdx; m++ {
				for j := 0; j < len(nums); j++ {
					if bs(nums[j], candi[m]) {
						count[j]--
					}
				}
				if !isHit(count) {
					t = m
					break
				}
			}
			if candi[eIdx]-candi[t] < minInterval {
				minInterval = candi[eIdx] - candi[t]
				ret = []int{candi[t], candi[eIdx]}
			}
			sIdx = t + 1
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
