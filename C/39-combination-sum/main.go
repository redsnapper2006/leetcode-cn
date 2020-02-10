package main

import (
	"fmt"
)

func combinationSum(candidates []int, target int) [][]int {
	buf := map[int][][]int{}
	buf[target] = [][]int{{target}}

	for i := target; i >= 0; i-- {
		v, ok := buf[i]

		if ok {
			for _, c := range candidates {
				b := make([][]int, len(v)+len(buf[i-c]))
				idx := 0
				for _, ll := range buf[i-c] {
					b[idx] = ll
					idx++
				}
				for _, vv := range v {
					if c <= vv[0] {
						t := make([]int, len(vv)+1)
						copy(t[1:], vv)
						t[0] = c
						b[idx] = t
						idx++
					}
				}
				buf[i-c] = b[0:idx]
			}
		}
	}

	for i := 0; i < len(buf[0]); i++ {
		buf[0][i] = buf[0][i][0 : len(buf[0][i])-1]
	}
	return buf[0]
}

func combinationSumV2(candidates []int, target int) [][]int {
	buf := map[int][][]int{}
	buf[0] = [][]int{{0}}

	for i := 0; i <= target; i++ {
		v, ok := buf[i]

		if ok {
			for _, c := range candidates {
				b := make([][]int, len(v)+len(buf[i+c]))
				idx := 0
				for _, ll := range buf[i+c] {
					b[idx] = ll
					idx++
				}
				for _, vv := range v {
					if c >= vv[len(vv)-1] {
						t := make([]int, len(vv)+1)
						copy(t, vv)
						t[len(vv)] = c
						b[idx] = t
						idx++
					}
				}
				buf[i+c] = b[0:idx]
			}
		}
	}

	for i := 0; i < len(buf[target]); i++ {
		buf[target][i] = buf[target][i][1:]
	}
	return buf[target]
}

func main() {
	fmt.Println(combinationSum([]int{2, 3, 5}, 8))
}
