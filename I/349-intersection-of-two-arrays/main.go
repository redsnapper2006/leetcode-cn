package main

import (
	"fmt"
)

func intersection(nums1 []int, nums2 []int) []int {
	var base, loop []int

	if len(nums1) > len(nums2) {
		base = nums2
		loop = nums1
	} else {
		base = nums1
		loop = nums2
	}

	buf := make(map[int]int)
	for i := 0; i < len(base); i++ {
		buf[base[i]]++
	}

	m := make(map[int]int)
	for i := 0; i < len(loop); i++ {
		_, ok := buf[loop[i]]
		if ok {
			m[loop[i]]++
		}
	}
	var r []int
	for k, _ := range m {
		r = append(r, k)
	}
	return r
}

func main() {
	fmt.Println(intersection([]int{1, 2, 2, 1}, []int{2, 2}))

	fmt.Println(intersection([]int{4, 9, 5}, []int{9, 4, 9, 8, 4}))

}
