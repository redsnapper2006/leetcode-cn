package main

import (
	"fmt"
	"sort"
)

func topKFrequent(nums []int, k int) []int {
	sort.Ints(nums)
	retCandi := make([]int, k)
	retCount := make([]int, k)
	cur := nums[0]
	count := 0
	for i, v := range nums {
		if v == cur {
			count++
		} else {
			min := count

			index := -1
			for j := 0; j < k; j++ {
				if retCount[j] < min {
					index = j
					min = retCount[j]
				}
			}

			if index > -1 {
				retCandi[index] = cur
				retCount[index] = count
			}
			cur = v
			count = 1
		}

		if i == len(nums)-1 {
			min := count
			index := -1
			for j := 0; j < k; j++ {
				if retCount[j] < min {
					index = j
					min = retCount[j]
				}
			}
			if index > -1 {
				retCandi[index] = cur
				retCount[index] = count
			}
		}
	}
	var ret []int
	for i, v := range retCount {
		if v > 0 {
			ret = append(ret, retCandi[i])
		}
	}
	return ret
}

func main() {
	fmt.Println(topKFrequent([]int{1, 1, 1, 2, 2, 3, 3, 3, 3}, 2))
}
