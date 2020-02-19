package main

import "fmt"

func helper(nums []int) int {
	s, e := 0, len(nums)-1
	for s < e {
		for s < e && nums[s] < 1 {
			s++
		}
		for e > s && nums[e] >= 1 {
			e--
		}
		if s < e {
			nums[s], nums[e] = nums[e], nums[s]
			s++
			e--
		}
	}
	if nums[s] == 0 {
		return s + 1
	}
	return s
}
func sort(nums []int, x, y int) {
	s, e := 0, len(nums)-1
	for s < e {
		for s < e && nums[s] == x {
			s++
		}
		for e > s && nums[e] == y {
			e--
		}

		if s < e {
			nums[s], nums[e] = nums[e], nums[s]
			s++
			e--
		}
	}
}
func sortColorsV2(nums []int) {
	mid := helper(nums)
	sort(nums[0:mid], 0, 1)
	sort(nums[mid:], 1, 2)
	fmt.Println(nums)
}

func sortColors(nums []int) {
	s0, e2 := 0, len(nums)-1

	for i := 0; i <= e2; i++ {
		if nums[i] == 0 {
			nums[s0], nums[i] = nums[i], nums[s0]
			s0++
		}
		if nums[i] == 2 {
			nums[e2], nums[i] = nums[i], nums[e2]
			e2--
			i--

		}
	}
	fmt.Println(nums)
}

func main() {
	sortColors([]int{2, 0, 1})
	sortColors([]int{0, 0})
	sortColors([]int{1, 0, 0})
	sortColors([]int{2, 0, 2, 1, 1, 0})
	sortColors([]int{1, 1, 2, 0, 2, 1, 0, 0, 2, 2})
}
