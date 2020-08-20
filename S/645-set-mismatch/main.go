package main

import "fmt"

func findErrorNums(nums []int) []int {
	m := map[int]int{}
	for i := 0; i < len(nums); i++ {
		m[nums[i]]++
	}
	miss := -1
	dul := -1
	for i := 1; i <= len(nums); i++ {
		v, ok := m[i]
		if !ok {
			miss = i
		} else if v == 2 {

			dul = i
		}
	}
	return []int{dul, miss}
}

func main() {
	fmt.Println("a")
}
