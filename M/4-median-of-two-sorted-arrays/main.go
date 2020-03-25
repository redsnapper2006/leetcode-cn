package main

import (
	"fmt"
)

func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {

	p1, p2 := nums1, nums2
	if len(p1) > len(p2) {
		p1, p2 = p2, p1
	}

	s1, e1 := 0, len(p1)
	for s1 <= e1 {
		m1 := (s1 + e1) / 2
		m2 := (len(p1)+len(p2)+1)/2 - m1
		if m1 < len(p1) && p2[m2-1] > p1[m1] {
			s1 = m1 + 1
		} else if m1 > s1 && p1[m1-1] > p2[m2] {
			e1 = m1 - 1
		} else {
			var l int
			if m1 == 0 {
				l = p2[m2-1]
			} else if m2 == 0 {
				l = p1[m1-1]
			} else {
				if p1[m1-1] > p2[m2-1] {
					l = p1[m1-1]
				} else {
					l = p2[m2-1]
				}
			}
			if (len(p1)+len(p2))%2 == 1 {
				return float64(l)
			}
			var r int
			if m1 == len(p1) {
				r = p2[m2]
			} else if m2 == len(p2) {
				r = p1[m1]
			} else {
				if p1[m1] < p2[m2] {
					r = p1[m1]
				} else {
					r = p2[m2]
				}
			}
			return float64(l+r) / 2
		}
	}

	return 0.0
}

func main() {
	fmt.Println(findMedianSortedArrays([]int{1, 2}, []int{3, 4}))
	fmt.Println(findMedianSortedArrays([]int{1, 3}, []int{2}))
	fmt.Println(findMedianSortedArrays([]int{1, 2}, []int{-1, 3}))
	fmt.Println(findMedianSortedArrays([]int{1}, []int{1}))
}
