package main

import (
	"fmt"
	"sort"
)

type StrArr []string

func (p StrArr) Len() int {
	return len(p)
}

func (p StrArr) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p StrArr) Less(i, j int) bool {
	if len(p[i]) != len(p[j]) {
		return len(p[i]) < len(p[j])
	}
	for m := 0; m < len(p[i]); m++ {
		if p[i][m] != p[j][m] {
			return p[i][m] < p[j][m]
		}
	}
	return true
}

func kthLargestNumber(nums []string, k int) string {
	sort.Sort(StrArr(nums))

	return nums[len(nums)-k]
}

func main() {
	fmt.Println()
}
