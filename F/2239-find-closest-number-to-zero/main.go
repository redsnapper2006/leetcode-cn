package main

func findClosestNumber(nums []int) int {
	min := 1000000
	org := 0
	for _, vv := range nums {
		v := vv

		if v < 0 {
			v = -v
		}
		if v < min {
			min = v
			org = vv
		} else if v == min && org < vv {
			org = vv
		}
	}
	return org
}
