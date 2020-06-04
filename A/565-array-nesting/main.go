package main

import "fmt"

func arrayNesting(nums []int) int {
	buf := make([]int, len(nums))

	ret := 0
	for i := 0; i < len(nums); i++ {
		if buf[i] == 1 {
			continue
		}
		steps := 0
		n := i
		for {
			if buf[n] == 1 {
				break
			}
			buf[n] = 1
			n = nums[n]
			steps++
		}
		if steps > ret {
			ret = steps
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
