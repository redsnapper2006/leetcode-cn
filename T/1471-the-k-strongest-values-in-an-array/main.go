package main

import "sort"

func getStrongest(arr []int, k int) []int {
	sort.Ints(arr)
	idx := (len(arr) - 1) / 2

	ret := []int{}
	l, r := 0, len(arr)-1
	cnt := 0
	for cnt < k && l <= r {
		dl, dr := arr[l]-arr[idx], arr[r]-arr[idx]
		if dl < 0 {
			dl = -dl
		}
		if dr < 0 {
			dr = -dr
		}
		if dr >= dl {
			ret = append(ret, arr[r])
			r--
		} else {
			ret = append(ret, arr[l])
			l++
		}
		cnt++
	}
	return ret
}
