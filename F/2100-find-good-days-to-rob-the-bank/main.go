package main

import "fmt"

func goodDaysToRobBank(security []int, time int) []int {
	left := make([]int, len(security))
	right := make([]int, len(security))

	cnt := 0
	for i := 1; i < len(security); i++ {
		if security[i] <= security[i-1] {
			cnt++
		} else {
			cnt = 0
		}
		left[i] = cnt
	}
	cnt = 0
	for i := len(security) - 2; i >= 0; i-- {
		if security[i] <= security[i+1] {
			cnt++
		} else {
			cnt = 0
		}
		right[i] = cnt
	}
	ret := []int{}
	for i := 0; i < len(security); i++ {
		if left[i] >= time && right[i] >= time {
			ret = append(ret, i)
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
