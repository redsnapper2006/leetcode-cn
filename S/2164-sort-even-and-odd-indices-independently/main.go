package main

import "sort"

func sortEvenOdd(nums []int) []int {
	odd, even := []int{}, []int{}
	for i, v := range nums {
		if i%2 == 0 {
			even = append(even, v)
		} else {
			odd = append(odd, v)
		}
	}
	sort.Ints(odd)
	sort.Ints(even)
	ret := []int{}
	ie, io := 0, len(odd)
	for ie < len(even) && io >= 0 {
		ret = append(ret, even[ie], odd[io])
		ie++
		io--
	}
	if ie < len(even) {
		ret = append(ret, even[ie])
	}
	if io >= 0 {
		ret = append(ret, odd[io])
	}
	return ret
}
