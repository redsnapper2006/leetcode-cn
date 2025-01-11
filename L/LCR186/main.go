package main

import "fmt"

func isStraight(nums []int) bool {
	buf := make([]int, 14)
	for i := 0; i < len(nums); i++ {
		buf[nums[i]]++
	}
	for i := 1; i < 12; i++ {
		if buf[i] == 1 {
			c := 0
			for j := i; j < i+5 && j < len(buf); j++ {
				if buf[j] == 1 {
					c++
				}
			}
			if c+buf[0] >= 5 {
				return true
			}
		}
	}
	return false
}

func main() {
	fmt.Println("a")
}
