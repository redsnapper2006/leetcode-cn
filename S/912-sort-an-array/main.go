package main

import "fmt"

func sortArray(nums []int) []int {
	var merge func(nums []int) []int
	merge = func(nums []int) []int {
		if len(nums) == 1 {
			return nums
		}
		if len(nums) == 2 {
			if nums[0] > nums[1] {
				return []int{nums[1], nums[0]}
			}
			return nums
		}

		size := len(nums)
		left := merge(nums[0 : size/2])
		right := merge(nums[size/2:])
		b := make([]int, size)
		ls, le, rs, re := 0, len(left), 0, len(right)
		idx := 0
		for ls < le && rs < re {
			if left[ls] < right[rs] {
				b[idx] = left[ls]
				ls++
			} else {
				b[idx] = right[rs]
				rs++
			}
			idx++
		}
		if ls < le {
			for i := ls; i < le; i++ {
				b[idx] = left[i]
				idx++
			}
		}
		if rs < re {
			for i := rs; i < re; i++ {
				b[idx] = right[i]
				idx++
			}
		}
		return b
	}
	return merge(nums)
}

func main() {
	fmt.Println(sortArray([]int{5, 2, 3, 1}))
	fmt.Println(sortArray([]int{5, 1, 1, 2, 0, 0}))

}
