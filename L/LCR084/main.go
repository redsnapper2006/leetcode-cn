package main

import (
	"fmt"
	"sort"
)

func permuteUnique(nums []int) [][]int {
	var dfs func(nums []int) [][]int
	dfs = func(nums []int) [][]int {
		if len(nums) == 1 {
			return [][]int{nums}
		}
		ret := [][]int{}
		sort.Ints(nums)
		for i := 0; i < len(nums); i++ {
			if i > 0 && nums[i] == nums[i-1] {
				continue
			}
			next := []int{}
			for j := 0; j < len(nums); j++ {
				if i == j {
					continue
				}
				next = append(next, nums[j])
			}

			candi := dfs(next)

			t := make([][]int, len(ret)+len(candi))
			copy(t, ret)

			for m := 0; m < len(candi); m++ {
				t[len(ret)+m] = make([]int, len(candi[m])+1)
				t[len(ret)+m][0] = nums[i]
				copy(t[len(ret)+m][1:], candi[m])
			}
			ret = t
		}
		return ret
	}
	return dfs(nums)
}

func main() {
	fmt.Println()
}
