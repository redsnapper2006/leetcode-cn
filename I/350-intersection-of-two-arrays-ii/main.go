package main

import (
	"fmt"
)

func intersect(nums1 []int, nums2 []int) []int {

	var size int
	var t1, t2 []int
	if len(nums1) > len(nums2) {
		size = len(nums2)
		t1 = nums2
		t2 = nums1
	} else {
		size = len(nums1)
		t1 = nums1
		t2 = nums2
	}
	ret := make([]int, size)

	r1 := 0
	for i := 0; i < len(t1); i++ {
		for j := 0; j < len(t2); j++ {
			if t1[i] == t2[j] {
				ret[r1] = t1[i]
				r1++
				t := make([]int , len(t2)-1)
				copy(t,t2[0:j])
				copy(t[j:len(t)],t2[j+1:len(t2)])
				t2 = t
				break
			}
		}
	}
	r := make([]int ,r1)
	copy(r, ret[0:r1])
	return r
}

func main() {
	fmt.Println(intersect([]int{1, 2, 2, 1}, []int{2, 2}))
	fmt.Println(intersect([]int{4,9,5}, []int{9,4,9,8,4}))
	fmt.Println(intersect([]int{3,1,2}, []int{1,1}))
}
