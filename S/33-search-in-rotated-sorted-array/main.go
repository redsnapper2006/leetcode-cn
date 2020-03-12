package main

import "fmt"

func search(nums []int, target int) int {
	if len(nums) == 0 {
		return -1
	}
	if len(nums) == 1 {
		if nums[0] == target {
			return 0
		} else {
			return -1
		}
	}
	if len(nums) == 2 {
		if nums[0] == target {
			return 0
		} else if nums[1] == target {
			return 1
		} else {
			return -1
		}
	}
	s, e := 0, len(nums)-1

	for s < e {
		fmt.Println("s,e", s, e)
		mid := s + (e-s)/2
		fmt.Println("mid", mid, nums[mid], target)
		if nums[mid] == target {
			return mid
		}
		if nums[mid] > nums[s] {
			if nums[mid] > target && nums[s] <= target {
				e = mid - 1
			} else {
				s = mid + 1
			}
		} else if nums[mid] < nums[s] {
			if nums[mid] < target && nums[e] >= target {
				s = mid + 1
			} else {
				e = mid - 1
			}
		} else {
			if nums[s] == target {
				return s
			}
			if nums[e] == target {
				return e
			}
			break
		}
	}
	if nums[s] == target {
		return s
	}
	if nums[e] == target {
		return e
	}
	return -1
}

func main() {
	// fmt.Println(search([]int{5, 1, 2, 3, 4}, 1))
	fmt.Println(search([]int{4, 5, 6, 7, 0, 1, 2}, 0))

}
