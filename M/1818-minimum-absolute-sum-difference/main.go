package main

import (
	"fmt"
	"sort"
)

func minAbsoluteSumDiff(nums1 []int, nums2 []int) int {
	sum := 0
	buf := make([]int, len(nums1))
	for i := 0; i < len(nums1); i++ {
		diff := nums1[i] - nums2[i]
		if diff < 0 {
			diff = -diff
		}
		sum += diff
		buf[i] = diff
	}
	if sum == 0 {
		return sum % 1000000007
	}
	// fmt.Println(sum)

	min := sum
	sort.Ints(nums1)
	for i := 0; i < len(nums1); i++ {
		base := nums2[i]

		s, e := 0, len(nums1)-1
		for s <= e {
			m := s + (e-s)/2
			if nums1[m] > base {
				e = m - 1
			} else if nums1[m] < base {
				s = m + 1
			} else {
				s = m
				break
			}
		}

		if s == len(nums1) {
			candi := nums1[len(nums1)-1]
			d := candi - base
			if d < 0 {
				d = -d
			}

			if sum-buf[i]+d < min {
				min = sum - buf[i] + d
			}
		} else if s == 0 {
			candi := nums1[0]
			d := candi - base
			if d < 0 {
				d = -d
			}

			if sum-buf[i]+d < min {
				min = sum - buf[i] + d
			}
		} else {
			d1 := nums1[s] - base
			if d1 < 0 {
				d1 = -d1
			}
			d2 := nums1[s-1] - base
			if d2 < 0 {
				d2 = -d2
			}
			d := d1
			if d > d2 {
				d = d2
			}

			if sum-buf[i]+d < min {
				min = sum - buf[i] + d
			}
		}
	}

	return min % 1000000007
}

func main() {
	fmt.Println(minAbsoluteSumDiff([]int{1, 10, 4, 4, 2, 7}, []int{9, 3, 5, 1, 7, 4}))
}
