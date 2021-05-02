package main

import "fmt"

func getMinDistance(nums []int, target int, start int) int {
	ret := len(nums) + 1
	for i := 0; i < len(nums); i++ {
		if nums[i] != target {
			continue
		}
		d := i - start
		if d < 0 {
			d = -d
		}
		if ret > d {
			ret = d
		}
	}
	return ret
}

func main() {
	fmt.Println("")
}
