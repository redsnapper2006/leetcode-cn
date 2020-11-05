package main

import "fmt"

func canFormArray(arr []int, pieces [][]int) bool {
	idx := 0
	for idx < len(arr) {
		pi := -1
		for i, p := range pieces {
			if p[0] == arr[idx] {
				pi = i
				break
			}
		}
		if pi == -1 {
			return false
		}
		for i := 0; i < len(pieces[pi]); i++ {
			if pieces[pi][i] == arr[idx] {
				idx++
			} else {
				return false
			}
		}
	}
	return true
}

func main() {
	fmt.Println("a")
}
