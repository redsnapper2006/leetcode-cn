package main

import "fmt"

func findShortestSubArray(nums []int) int {
	MC := map[int]int{}
	MS := map[int]int{}
	ME := map[int]int{}
	for i := 0; i < len(nums); i++ {
		b := nums[i]

		MC[b]++
		_, ok := MS[b]
		if !ok {
			MS[b] = i
			ME[b] = i
		} else {
			ME[b] = i
		}
	}
	max := -1
	for _, v := range MC {
		if v > max {
			max = v
		}
	}

	ret := len(nums)
	for k, v := range MC {
		if v == max {
			if ME[k]-MS[k] < ret {
				ret = ME[k] - MS[k]
			}
		}
	}
	return ret + 1
}

func main() {
	fmt.Println("a")
}
