package main

import (
	"fmt"
)

func plusOne(digits []int) []int {
	isPlus := false
	digits[len(digits)-1]++
	if digits[len(digits)-1] > 9 {
		digits[len(digits)-1] -= 10
		isPlus = true
	}

	if !isPlus {
		return digits
	} else {
		for i := len(digits) - 2; i >= 0; i-- {
			if isPlus {
				digits[i]++
			}
			if digits[i] > 9 {
				digits[i] -= 10
				isPlus = true
			} else {
				isPlus = false
			}
		}
		if isPlus {
			return append([]int{1}, digits...)
		} else {
			return digits
		}
	}
}

func main() {
	fmt.Println("a")
}
