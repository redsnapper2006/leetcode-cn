package main

import (
	"fmt"
	"sort"
)

func twoSum(n int) []float64 {
	buf := map[int]int{}
	for i := 1; i <= 6; i++ {
		buf[i]++
	}

	for i := 1; i < n; i++ {
		tmap := map[int]int{}
		for k, v := range buf {
			for m := 1; m <= 6; m++ {
				tmap[k+m] += v
			}
		}
		buf = tmap
	}
	var r []int
	sum := 0
	for k := range buf {
		r = append(r, k)
	}
	sort.Ints(r)
	for i := 0; i < len(r); i++ {
		sum += buf[r[i]]
	}
	var ret []float64
	for i := 0; i < len(r); i++ {
		ret = append(ret, float64(buf[r[i]])/float64(sum))
	}
	return ret
}

func main() {
	fmt.Println("a")
}
