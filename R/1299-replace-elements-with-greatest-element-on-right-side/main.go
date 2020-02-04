package main

import (
	"fmt"
)

func replaceElements(arr []int) []int {
	if len(arr) == 1 {
		return []int{-1}
	}

	max := -1
	for i := len(arr) - 1; i >= 0; i-- {

		t := arr[i]
		arr[i] = max
		if t > max {
			max = t
		}
	}
	return arr
}

func main() {
	fmt.Println(replaceElements([]int{17, 18, 5, 4, 6, 1}))
}
