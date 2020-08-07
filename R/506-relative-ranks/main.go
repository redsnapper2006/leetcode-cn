package main

import (
	"fmt"
	"sort"
	"strconv"
)

func findRelativeRanks(nums []int) []string {
	buf := make([]int, len(nums))
	copy(buf, nums)
	sort.Ints(buf)

	M := map[int]int{}
	for i := 0; i < len(buf); i++ {
		M[buf[i]] = i
	}

	var ret []string
	for i := 0; i < len(nums); i++ {
		if M[nums[i]] == len(nums)-1 {
			ret = append(ret, "Gold Medal")
		} else if M[nums[i]] == len(nums)-2 {
			ret = append(ret, "Silver Medal")
		} else if M[nums[i]] == len(nums)-3 {
			ret = append(ret, "Bronze Medal")
		} else {
			rank := strconv.FormatInt(int64(len(nums)-M[nums[i]]), 10)
			ret = append(ret, rank)
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
