package main

import (
	"fmt"
)

func deleteAndEarn(nums []int) int {
	buf := make([]int, 10001)
	for _, v := range nums {
		buf[v]++
	}
	ret := make([]int, 10001)
	for i := 0; i < 10001; i++ {
		p2, p3 := 0, 0
		if i >= 2 {
			p2 = ret[i-2]
		}
		if i >= 3 {
			p3 = ret[i-3]
		}
		p := p2
		if p < p3 {
			p = p3
		}
		ret[i] = i*buf[i] + p
	}

	v := ret[len(ret)-2]
	if v < ret[len(ret)-1] {
		v = ret[len(ret)-1]
	}
	return v
}

func main() {
	fmt.Println()
}
