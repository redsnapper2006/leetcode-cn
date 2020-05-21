package main

import (
	"fmt"
	"sort"
)

type IntArrSlice [][]int

func (p IntArrSlice) Len() int {
	return len(p)
}

func (p IntArrSlice) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p IntArrSlice) Less(i, j int) bool {
	return p[i][0]+p[i][1] < p[j][0]+p[j][1]
}

func kSmallestPairs(nums1 []int, nums2 []int, k int) [][]int {
	if len(nums1) == 0 || len(nums2) == 0 || k == 0 {
		return nil
	}

	var ret [][]int
	for i := 0; i < k; i++ {
		for j := 0; j <= i; j++ {
			if j < len(nums1) && (i-j) < len(nums2) {
				ret = append(ret, []int{nums1[j], nums2[i-j]})
			}
		}
	}
	if len(ret) <= k {
		return ret
	}
	sort.Sort(IntArrSlice(ret))
	return ret[0:k]
}

func main() {
	fmt.Println("a")
}
