package main

import "fmt"

func singleNumber(nums []int) int {
	buf := make([]int, 32)
	for i := 0; i < len(nums); i++ {
		for j := 0; j < 32; j++ {
			if nums[i]&(1<<j) != 0 {
				buf[j]++
			}
		}
	}
	s := 0
	for i := 0; i < len(buf); i++ {
		if buf[i]%3 != 0 {
			s += (1 << i)
		}
	}
	return s
}

func main() {
	fmt.Println("a")
}
