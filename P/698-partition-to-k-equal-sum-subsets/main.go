package main

import (
	"fmt"
	"sort"
)

func canPartitionKSubsets(nums []int, k int) bool {
	sum := 0
	for _, n := range nums {
		sum += n
	}
	if sum%k != 0 {
		return false
	}

	gCnt := sum / k
	sort.Ints(nums)
	if nums[len(nums)-1] > gCnt {
		return false
	}
	group := make([]int, k)

	var dfs func(nums []int, nIdx int, group []int) bool
	dfs = func(nums []int, nIdx int, group []int) bool {
		if nIdx == len(nums) {
			return true
		}

		for i := 0; i < k; i++ {
			if i > 0 && group[i] == group[i-1] {
				continue
			}
			if group[i]+nums[nIdx] > gCnt {
				continue
			}
			group[i] += nums[nIdx]
			if dfs(nums, nIdx+1, group) {
				return true
			}
			group[i] -= nums[nIdx]
		}
		return false
	}

	return dfs(nums, 0, group)
}

func main() {
	fmt.Println(canPartitionKSubsets([]int{1739, 5391, 8247, 236, 5581, 11, 938,
		58, 1884, 823, 686, 1760, 6498, 6513, 6316, 2867}, 6))
}
