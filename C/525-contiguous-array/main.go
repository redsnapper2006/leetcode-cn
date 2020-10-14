package main

import "fmt"

func findMaxLength(nums []int) int {
	M := map[int]int{0: -1}

	zeroCnt, oneCnt := 0, 0
	max := 0
	for i, v := range nums {
		if v == 0 {
			zeroCnt++
		} else {
			oneCnt++
		}
		idx, ok := M[zeroCnt-oneCnt]
		if !ok {
			M[zeroCnt-oneCnt] = i
		} else {
			if i-idx > max {
				max = i - idx
			}
		}
	}
	if zeroCnt == oneCnt {
		return len(nums)
	}
	return max
}

func main() {
	fmt.Println("a")
}
