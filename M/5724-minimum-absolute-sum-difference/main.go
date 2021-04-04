package main

import "fmt"

func minAbsoluteSumDiff(nums1 []int, nums2 []int) int {
	sum := 0
	max := -1
	maxIdx := -1
	for i := 0; i < len(nums1); i++ {
		diff := nums1[i] - nums2[i]
		if diff < 0 {
			diff = -diff
		}
		if max < diff {
			max = diff
			maxIdx = i
		}
		sum += diff
	}
	if max == 0 {
		return sum % 1000000007
	}

	newD := max
	for i := 0; i < len(nums1); i++ {
		if i == maxIdx {
			continue
		}
		d := nums1[i] - nums2[maxIdx]
		if d < 0 {
			d = -d
		}
		if newD > d {
			newD = d
		}
	}

	return (sum - max + newD) % 1000000007
}

func main() {
	fmt.Println(minAbsoluteSumDiff([]int{1, 10, 4, 4, 2, 7}, []int{9, 3, 5, 1, 7, 4}))
}
