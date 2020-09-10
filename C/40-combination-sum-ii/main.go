package main

import (
	"fmt"
	"sort"
)

func combinationSum2(candidates []int, target int) [][]int {
	sort.Ints(candidates)
	buf := make([][][][]int, len(candidates))
	for i, c := range candidates {
		buf[i] = make([][][]int, target+1)
		if c > target {
			continue
		}
		if i > 0 && candidates[i-1] == c {
			continue
		}
		buf[i][c] = [][]int{{c}}
	}

	for i := range candidates {
		for j := 0; j <= target; j++ {
			// if j == c && i > 0 && candidates[i-1] == c {
			// 	continue
			// }
			if buf[i][j] != nil {
				arr := buf[i][j]
				for m := i + 1; m < len(candidates); m++ {
					if j+candidates[m] > target {
						break
					}
					if m > i+1 && candidates[m] == candidates[m-1] {
						continue
					}
					for k := 0; k < len(arr); k++ {
						t := arr[k]
						tt := make([]int, len(t)+1)
						copy(tt, t)
						tt[len(tt)-1] = candidates[m]
						buf[m][j+candidates[m]] = append(buf[m][j+candidates[m]], tt)
					}
				}
			}
		}
	}

	var ret [][]int
	for _, v := range buf {
		if v[target] != nil {
			for _, b := range v[target] {
				ret = append(ret, b)
			}
		}
	}
	return ret
}

func main() {
	fmt.Println(combinationSum2([]int{10, 1, 2, 7, 6, 1, 5}, 8))
	fmt.Println(combinationSum2([]int{2, 5, 2, 1, 2}, 5))
	fmt.Println(combinationSum2([]int{1, 1}, 1))
}
