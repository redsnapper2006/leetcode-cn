package main

import (
	"fmt"
	"strconv"
)

func summaryRanges(nums []int) []string {
	if len(nums) == 0 {
		return nil
	}
	var ret []string
	start := -1
	prev := nums[0] - 2
	for i := 0; i < len(nums); i++ {
		if nums[i] != prev+1 {
			if i != 0 {
				if start == prev {
					ret = append(ret, strconv.FormatInt(int64(start), 10))
				} else {
					ret = append(ret, strconv.FormatInt(int64(start), 10)+"->"+strconv.FormatInt(int64(prev), 10))
				}
			}
			start = nums[i]
		}
		prev = nums[i]
	}
	if start == prev {
		ret = append(ret, strconv.FormatInt(int64(start), 10))
	} else {
		ret = append(ret, strconv.FormatInt(int64(start), 10)+"->"+strconv.FormatInt(int64(prev), 10))
	}
	return ret
}

func main() {
	fmt.Println("a")
}
