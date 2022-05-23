package main

import "sort"

func largestInteger(num int) int {
	odd, even := []int{}, []int{}
	n := num
	buf := []int{}
	for n > 0 {
		oe := n % 10
		if oe%2 == 0 {
			even = append(even, oe)
		} else {
			odd = append(odd, oe)
		}
		buf = append(buf, oe)
		n /= 10
	}
	sort.Ints(odd)
	sort.Ints(even)
	oidx, eidx := len(odd)-1, len(even)-1

	cnt := 0
	for i := len(buf) - 1; i >= 0; i-- {
		if buf[i]%2 == 0 {
			cnt = cnt*10 + even[eidx]
			eidx--
		} else {
			cnt = cnt*10 + odd[oidx]
		}
	}
	return cnt
}
