package main

import "fmt"

func twoOutOfThree(nums1 []int, nums2 []int, nums3 []int) []int {
	m1, m2, m3 := map[int]int{}, map[int]int{}, map[int]int{}
	for _, v := range nums1 {
		m1[v] = 1
	}
	for _, v := range nums2 {
		m2[v] = 1
	}
	for _, v := range nums3 {
		m3[v] = 1
	}
	total := map[int]int{}
	for k := range m1 {
		total[k]++
	}
	for k := range m2 {
		total[k]++
	}
	for k := range m3 {
		total[k]++
	}
	ret := []int{}
	for k, v := range total {
		if v >= 2 {
			ret = append(ret, k)
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
