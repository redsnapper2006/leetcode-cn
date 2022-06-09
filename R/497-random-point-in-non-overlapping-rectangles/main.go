package main

import (
	"math/rand"
	"sort"
)

type Solution struct {
	R [][]int
	G []int
}

func Constructor(rects [][]int) Solution {
	g := make([]int, len(rects)+1)
	for i, rect := range rects {
		g[i+1] = g[i] + (rect[2]-rect[0]+1)*(rect[3]-rect[1]+1)
	}
	return Solution{R: rects, G: g}
}

func (this *Solution) Pick() []int {

	idx := rand.Intn(this.G[len(this.G)-1])
	rectIndex := sort.SearchInts(this.G, idx+1) - 1
	r := this.R[rectIndex]
	w := (idx - this.G[rectIndex]) / (r[2] - r[0] + 1)
	h := (idx - this.G[rectIndex]) % (r[2] - r[0] + 1)
	return []int{r[0] + h, r[1] + w}
}
