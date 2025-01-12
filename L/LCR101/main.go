package main

import "fmt"

func canPartition(nums []int) bool {
	sum := 0
	for _, v := range nums {
		sum += v
	}
	if sum%2 == 1 {
		return false
	}
	target := sum / 2
	if nums[0] == target {
		return true
	}

	buf := make([]int, target+1)
	buf[0] = 1
	candi := []int{0}
	for i := 0; i < len(nums); i++ {
		size := len(candi)
		for j := 0; j < size; j++ {
			c := candi[j] + nums[i]
			if c == target {
				return true
			}
			if c > target {
				continue
			}
			if buf[c] == 1 {
				continue
			}
			buf[c] = 1
			candi = append(candi, c)
		}
	}
	return false
}

func main() {
	fmt.Println(canPartition([]int{23, 13, 11, 7, 6, 5, 5}))
}
