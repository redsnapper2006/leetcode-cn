package main

import "math/rand"

type Solution struct {
	M map[int]int
	C int
}

func Constructor(n int, blacklist []int) Solution {
	C := n - len(blacklist)
	m := map[int]int{}
	for _, v := range blacklist {
		m[v] = 1
	}
	r := C
	nm := map[int]int{}
	for _, b := range blacklist {
		if b < C {
			for m[r] > 0 {
				r++
			}
			nm[b] = r
			r++
		}
	}
	return Solution{M: nm, C: C}
}

func (this *Solution) Pick() int {
	x := rand.Intn(this.C)
	if this.M[x] > 0 {
		return this.M[x]
	}
	return x
}
