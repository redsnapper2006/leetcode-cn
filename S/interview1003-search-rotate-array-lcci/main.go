package main

import "fmt"

func search(arr []int, target int) int {
	var recur func(arr []int, target int) int
	recur = func(arr []int, target int) int {
		if len(arr) == 1 {
			if arr[0] == target {
				return 0
			}
			return -1
		}
		if len(arr) == 2 {
			if arr[0] == target {
				return 0
			}
			if arr[1] == target {
				return 1
			}
			return -1
		}

		s, e := 0, len(arr)-1
		m := s + (e-s)/2
		mIdx := -1
		if arr[m] == target {
			mIdx = m
		}
		left := recur(arr[0:m], target)
		right := recur(arr[m+1:], target)
		r := -1
		if right != -1 {
			r = right + m + 1
		}
		if mIdx != -1 {
			r = mIdx
		}
		if left != -1 {
			r = left
		}
		return r
	}
	return recur(arr, target)
}

func main() {
	fmt.Println("a")
}
