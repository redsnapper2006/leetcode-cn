package main

import "fmt"

func findBestValue(arr []int, target int) int {
	sum := 0
	max := 0
	for i := 0; i < len(arr); i++ {
		sum += arr[i]
		if max < arr[i] {
			max = arr[i]
		}
	}
	if sum < target {
		return max
	}

	s, e := 0, target
	minus := 1<<31 - 1
	ret := target
	for s <= e {
		m := s + (e-s)/2

		sum := 0
		for i := 0; i < len(arr); i++ {
			if arr[i] > m {
				sum += m
			} else {
				sum += arr[i]
			}
		}
		if sum == target {
			return m
		}
		ex := sum - target
		if ex < 0 {
			ex = -ex
		}
		if ex < minus {
			minus = ex
			ret = m
		}
		if sum < target {
			s = m + 1
		} else {
			e = m - 1
		}
	}
	return ret
}

func main() {
	fmt.Println(findBestValue([]int{2, 3, 5}, 1))
}
