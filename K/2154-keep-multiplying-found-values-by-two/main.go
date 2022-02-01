package main

import "fmt"

func findFinalValue(nums []int, original int) int {
	m := map[int]int{}
	for _, v := range nums {
		m[v] = 1
	}

	o := original
	for {
		_, ok := m[o]
		if ok {
			o *= 2
		} else {
			return o
		}
	}
}

func main() {
	fmt.Println()
}
