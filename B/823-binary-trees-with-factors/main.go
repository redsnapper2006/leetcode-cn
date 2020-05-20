package main

import (
	"fmt"
	"sort"
)

func numFactoredBinaryTrees(A []int) int {
	sort.Ints(A)

	buf := make(map[int][][]int)
	for i := 0; i < len(A); i++ {
		buf[A[i]] = [][]int{{A[i]}}
	}

	for i := 0; i < len(A); i++ {
		for j := 0; j < len(A); j++ {
			_, ok := buf[A[i]*A[j]]
			if !ok {
				continue
			}
			buf[A[i]*A[j]] = append(buf[A[i]*A[j]], []int{A[i], A[j]})
		}
	}

	M := map[int]int{}
	for i := 0; i < len(A); i++ {
		candi := buf[A[i]]
		for j := 0; j < len(candi); j++ {
			if len(candi[j]) == 1 {
				M[A[i]]++
			} else {
				multi := 1
				for m := 0; m < len(candi[j]); m++ {
					multi *= M[candi[j][m]]
					multi %= 1000000007
				}
				M[A[i]] += multi
			}
		}
	}
	ret := 0
	for _, v := range M {
		ret += v
		ret %= 1000000007
	}
	return ret
}

func main() {
	fmt.Println(numFactoredBinaryTrees([]int{2, 4, 5, 10}))
}
