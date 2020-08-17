package main

import (
	"fmt"
	"sort"
)

func computeArea(A int, B int, C int, D int, E int, F int, G int, H int) int {
	overlap := 0
	if C < E || G < A || D < F || H < B {
		overlap = 0
	} else {
		h := []int{A, C, E, G}
		v := []int{B, D, F, H}

		sort.Ints(h)
		sort.Ints(v)
		overlap = (h[2] - h[1]) * (v[2] - v[1])
	}

	return (C-A)*(D-B) + (G-E)*(H-F) - overlap
}

func main() {
	fmt.Println("a")
}
