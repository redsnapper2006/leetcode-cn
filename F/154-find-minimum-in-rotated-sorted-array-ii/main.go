package main

import "fmt"

func findMin(nums []int) int {
	s, e := 0, len(nums)-1
	for s < e {
		m := s + (e-s)/2
		if nums[m] < nums[e] {
			e = m
		} else if nums[m] > nums[e] {
			s = m + 1
		} else {
			e--
		}
	}
	return nums[s]
}

func findMinV2(nums []int) int {
	if len(nums) == 1 {
		return nums[0]
	}
	if len(nums) == 2 {
		if nums[0] > nums[1] {
			return nums[1]
		}
		return nums[0]
	}
	s, e := 0, len(nums)-1

	for s < e {
		m := s + (e-s)/2
		if nums[m] > nums[s] {
			if nums[m] > nums[e] {
				return findMin(nums[m+1 : e+1])
			} else {
				return nums[s]
			}
		} else if nums[m] < nums[s] {
			return findMin(nums[s : m+1])
		} else {
			if nums[m] < nums[e] {
				return nums[s]
			} else if nums[m] > nums[e] {
				return findMin(nums[m+1 : e+1])
			} else {
				left := findMin(nums[s:m])
				right := findMin(nums[m : e+1])
				if left < right {
					return left
				} else {
					return right
				}
			}
		}
	}
	return nums[s]
}

func main() {
	fmt.Println(findMin([]int{1, 3, 3}))
	fmt.Println(findMin([]int{3, 3, 1, 3}))
	fmt.Println(findMin([]int{1, 3, 1, 1}))
	fmt.Println(findMin([]int{10, 1, 10, 10, 10}))
}
