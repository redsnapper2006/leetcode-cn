package main

import (
	"fmt"
)

func majorityElement(nums []int) int {
	accum := make(map[int]int)
	for _, v := range nums {
		accum[v]++
		if accum[v] >= (len(nums)+1)/2 {
			return v
		}
	}
	// for k, v := range accum {
	// 	if v >= (len(nums)+1)/2 {
	// 		return k
	// 	}
	// }
	return 0
}

func main() {
	fmt.Println(majorityElement([]int{3, 2, 3}) == 3)
	fmt.Println(majorityElement([]int{2, 2, 1, 1, 1, 2, 2}) == 2)

}
