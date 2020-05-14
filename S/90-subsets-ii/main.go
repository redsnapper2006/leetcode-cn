package main

import (
	"fmt"
	"sort"
)

func subsetsWithDup(nums []int) [][]int {
	sort.Ints(nums)
	ret := [][]int{[]int{}, []int{nums[len(nums)-1]}}

	for i := len(nums) - 2; i >= 0; i-- {
		var t [][]int
		for j := 0; j < len(ret); j++ {
			t = append(t, ret[j])
		}
		if nums[i] != nums[i+1] {
			for j := 0; j < len(ret); j++ {
				t = append(t, append([]int{nums[i]}, ret[j]...))
			}
		} else {
			count := 0
			for m := i + 1; m < len(nums); m++ {
				if nums[m] == nums[i] {
					count++
				} else {
					break
				}
			}
			for j := 0; j < len(ret); j++ {
				if len(ret[j]) > 0 && ret[j][0] == nums[i] {
					tc := 0
					for n := 0; n < len(ret[j]); n++ {
						if ret[j][n] == nums[i] {
							tc++
						} else {
							break
						}
					}
					if tc == count {
						t = append(t, append([]int{nums[i]}, ret[j]...))
					}
				}
			}
		}
		ret = t
	}

	return ret
}

func main() {
	fmt.Println("a")
}
