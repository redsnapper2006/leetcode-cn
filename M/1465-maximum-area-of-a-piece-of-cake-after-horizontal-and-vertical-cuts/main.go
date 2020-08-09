package main

import (
	"fmt"
	"sort"
)

func maxArea(h int, w int, horizontalCuts []int, verticalCuts []int) int {
	horizontalCuts = append(horizontalCuts, h)
	verticalCuts = append(verticalCuts, w)
	sort.Ints(horizontalCuts)
	sort.Ints(verticalCuts)
	bh, bw := 0, 0
	hmax, wmax := 0, 0
	for i := 0; i < len(horizontalCuts); i++ {
		if horizontalCuts[i]-bh > hmax {
			hmax = horizontalCuts[i] - bh
		}
		bh = horizontalCuts[i]
	}

	for i := 0; i < len(verticalCuts); i++ {
		if verticalCuts[i]-bw > wmax {
			wmax = verticalCuts[i] - bw
		}
		bw = verticalCuts[i]
	}
	return (wmax * hmax) % 1000000007
}

func main() {
	fmt.Println("a")
}
