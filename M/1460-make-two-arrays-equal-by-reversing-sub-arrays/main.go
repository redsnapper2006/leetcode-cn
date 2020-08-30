package main

import (
	"fmt"
	"sort"
)

func canBeEqual(target []int, arr []int) bool {
	sort.Ints(arr)
	sort.Ints(target)
	for i := 0; i < len(target); i++ {
		if target[i] != arr[i] {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println("a")
}
