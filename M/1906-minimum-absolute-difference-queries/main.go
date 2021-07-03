package main

import "fmt"

func minDifference(nums []int, queries [][]int) []int {
	buf := make([][100]int, len(nums)+1)
	for i := 1; i <= len(nums); i++ {
		// buf[i] = make([]int, 100)
		buf[i] = buf[i-1]
		buf[i][nums[i-1]-1]++
	}
	ret := []int{}
	for i := 0; i < len(queries); i++ {
		s, e := queries[i][0], queries[i][1]+1
		idx1, idx2 := -1, -1
		diff := 101
		for j := 0; j < 100; j++ {
			if buf[e][j] == buf[s][j] {
				continue
			} else {
				idx2 = idx1
				idx1 = j
				if idx1 > -1 && idx2 > -1 && diff > idx1-idx2 {
					diff = idx1 - idx2
				}
			}
		}
		if diff == 101 {
			diff = -1
		}
		ret = append(ret, diff)
	}
	return ret
}

func main() {
	fmt.Println()
}
