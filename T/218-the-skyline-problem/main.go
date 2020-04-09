package main

import (
	"fmt"
	"sort"
)

func getSkyline(buildings [][]int) [][]int {
	if len(buildings) == 0 {
		return nil
	}
	if len(buildings) == 1 {
		return [][]int{{buildings[0][0], buildings[0][2]}, {buildings[0][1], 0}}
	}

	var cord []int
	for i := 0; i < len(buildings); i++ {
		cord = append(cord, buildings[i][0], buildings[i][1])
	}
	sort.Ints(cord)

	base := []int{cord[0]}
	for i := 1; i < len(cord); i++ {
		if cord[i] != base[len(base)-1] {
			base = append(base, cord[i])
		}
	}

	var ret [][]int
	for i := 0; i < len(base); i++ {
		b := base[i]
		max := 0
		for j := 0; j < len(buildings); j++ {
			candi := buildings[j]
			if candi[0] > b {
				break
			}
			if candi[1] <= b {
				continue
			}
			if candi[0] <= b && candi[1] > b && candi[2] > max {
				max = candi[2]
			}
		}

		if len(ret) == 0 || max != ret[len(ret)-1][1] {
			ret = append(ret, []int{b, max})
		}
	}

	return ret
}

func main() {
	fmt.Println(getSkyline([][]int{{1, 2, 1}, {2147483646, 2147483647, 2147483647}}))
	fmt.Println(getSkyline([][]int{{2, 9, 10}, {3, 7, 15}, {5, 12, 12}, {15, 20, 10}, {19, 24, 8}}))
}
