package main

import "sort"

func intersection(nums [][]int) []int {
	m := map[int]int{}
	for _, arr := range nums {
		for _, v := range arr {
			m[v]++
		}
	}
	ret := []int{}
	for k, v := range m {
		if v == len(nums) {
			ret = append(ret, k)
		}
	}
	sort.Ints(ret)
	return ret
}
