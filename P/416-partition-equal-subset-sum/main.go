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
	occupy := make([]int, target+1)
	occupy[0] = 1

	candi := []int{0}
	for _, v := range nums {
		size := len(candi)
		for i := 0; i < size; i++ {
			add := candi[i] + v
			if add == target {
				return true
			}
			if add > target || occupy[add] == 1 {
				continue
			}

			occupy[add] = 1
			candi = append(candi, add)
		}
	}
	return false
}

func main() {
	fmt.Println()
}
