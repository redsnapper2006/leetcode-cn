package main

import (
	"fmt"
	"sort"
)

func getKth(lo int, hi int, k int) int {
	M := map[int][]int{}
	sarr := []int{}
	for i := lo; i <= hi; i++ {
		b := i
		steps := 0
		for b > 1 {
			if b%2 == 0 {
				b /= 2
			} else {
				b = 3*b + 1
			}
			steps++
		}
		_, ok := M[steps]
		if !ok {
			sarr = append(sarr, steps)
			M[steps] = []int{}
		}
		M[steps] = append(M[steps], i)
	}

	sort.Ints(sarr)
	var s []int
	for _, v := range sarr {
		cnt := M[v]
		if len(cnt) >= k {
			s = cnt
			break
		}
		k -= len(cnt)
	}

	return s[k-1]
}

func main() {
	fmt.Println()
}
