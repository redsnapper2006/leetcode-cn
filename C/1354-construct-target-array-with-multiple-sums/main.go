package main

import (
	"fmt"
)

func valid(t []int) (bool, bool) {
	isValid := true
	for i := 0; i < len(t); i++ {
		if t[i] > 0 {
			if t[i] != 1 {
				isValid = false
			}
		} else {
			return false, true
		}
	}
	return isValid, false
}

func isPossible(target []int) bool {
	if len(target) == 1 {
		if target[0] == 1 {
			return true
		} else {
			return false
		}
	}

	isValid, isEnd := valid(target)
	if isEnd {
		return false
	}
	if isValid {
		return true
	}

	max1 := 0
	accum := 0
	for i := 0; i < len(target); i++ {
		if target[i] > target[max1] {
			max1 = i
		}
		accum += target[i]
	}

	accum -= target[max1]
	if accum > 1 {
		if target[max1] < accum {
			return false
		}
		target[max1] %= accum
	} else {
		target[max1] = 1
	}

	return isPossible(target)
}

func main() {
	// fmt.Println(isPossible([]int{1, 1000000000}))
	// fmt.Println(isPossible([]int{9, 3, 5}))
	// fmt.Println(isPossible([]int{9, 9, 9}))

	fmt.Println(isPossible([]int{2, 900000001}))
	fmt.Println(isPossible([]int{8, 5}))
}
