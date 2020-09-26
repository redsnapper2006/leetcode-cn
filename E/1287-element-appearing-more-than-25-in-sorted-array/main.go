package main

import "fmt"

func findSpecialInteger(arr []int) int {
	total := len(arr)
	cur := -1
	cnt := 0
	for i := 0; i < len(arr); i++ {
		if cur == arr[i] {
			cnt++
		} else {
			if cnt*4 > total {
				return arr[i-1]
			}
			cur = arr[i]
			cnt = 1
		}
	}
	if cnt*4 > total {
		return arr[len(arr)-1]
	}
	return -1
}

func main() {
	fmt.Println("a")
}
