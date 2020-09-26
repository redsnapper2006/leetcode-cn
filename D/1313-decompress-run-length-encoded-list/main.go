package main

import "fmt"

func decompressRLElist(nums []int) []int {
	var buf []int
	for i := 0; i < len(nums); i = i + 2 {
		c := nums[i]
		n := nums[i+1]
		for j := 0; j < c; j++ {
			buf = append(buf, n)
		}
	}
	return buf
}

func main() {
	fmt.Println("a")
}
