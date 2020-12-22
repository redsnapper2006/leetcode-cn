package main

import (
	"fmt"
	"sort"
)

func checkArithmeticSubarrays(nums []int, l []int, r []int) []bool {
	var ret []bool
	for i := 0; i < len(l); i++ {
		s, e := l[i], r[i]
		buf := make([]int, e-s+1)
		copy(buf, nums[s:e+1])
		sort.Ints(buf)
		isAri := true
		diff := buf[1] - buf[0]
		for j := 2; j < len(buf); j++ {
			if buf[j]-buf[j-1] != diff {
				isAri = false
				break
			}
		}
		ret = append(ret, isAri)
	}
	return ret
}

func main() {
	fmt.Println()
}
