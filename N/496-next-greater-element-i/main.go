package main

import "fmt"

func nextGreaterElement(nums1 []int, nums2 []int) []int {
	if len(nums1) == 0 || len(nums2) == 0 {
		return nil
	}
	buf := make([]int, len(nums2))
	stack := []int{nums2[0]}
	pos := []int{0}
	for i := 1; i < len(nums2); i++ {
		if nums2[i] > stack[len(stack)-1] {
			idx := len(stack) - 1
			for idx >= 0 && stack[idx] < nums2[i] {
				buf[pos[idx]] = nums2[i]
				idx--
			}
			stack = stack[0 : idx+1]
			pos = pos[0 : idx+1]
		}

		stack = append(stack, nums2[i])
		pos = append(pos, i)
	}

	for i := 0; i < len(pos); i++ {
		buf[pos[i]] = -1
	}
	M := map[int]int{}
	for i := 0; i < len(nums2); i++ {
		M[nums2[i]] = buf[i]
	}
	var ret []int
	for i := 0; i < len(nums1); i++ {
		ret = append(ret, M[nums1[i]])
	}
	return ret
}

func main() {
	fmt.Println("a")
}
