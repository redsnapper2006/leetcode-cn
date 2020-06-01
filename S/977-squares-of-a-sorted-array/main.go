package main

import (
	"fmt"
	"sort"
)

func sortedSquares(A []int) []int {
	ret := make([]int, len(A))
	s, e := 0, len(A)-1
	idx := len(A) - 1
	for s <= e {
		if A[s]*A[s] > A[e]*A[e] {
			ret[idx] = A[s] * A[s]
			s++
		} else {
			ret[idx] = A[e] * A[e]
			e--
		}
		idx--
	}
	return ret
}

func sortedSquaresV2(A []int) []int {
	var ret []int
	for i := 0; i < len(A); i++ {
		ret = append(ret, A[i]*A[i])
	}
	sort.Ints(ret)

	return ret
}

func main() {
	fmt.Println("a")
}
