package main

import "fmt"

func findDisappearedNumbers(nums []int) []int {
	var ret []int

	for i := 0; i < len(nums); i++ {
		if nums[i] == i+1 {
			continue
		}
		s := i
		p := 0
		fmt.Println(nums, s)
		for s >= 0 && nums[s] != s+1 {
			t := nums[s]
			nums[s] = p
			if t == 0 {
				break
			}
			n := nums[t-1]
			nums[t-1] = t
			p = n
			s = n - 1
			fmt.Println("loop", nums, s)
		}
	}

	for i := 0; i < len(nums); i++ {
		if nums[i] == 0 {
			ret = append(ret, i+1)
		}
	}

	return ret
}

func main() {
	// fmt.Println(findDisappearedNumbers([]int{4, 3, 2, 7, 8, 2, 3, 1}))
	fmt.Println(findDisappearedNumbers([]int{1, 1, 2, 2}))
}
