package main

import "fmt"

func maximumPrimeDifference(nums []int) int {
	min, max := -1, -1
	for i, n := range nums {
		if n < 2 {
			continue
		}
		isPrime := false
		for j := 2; j*j <= n; j++ {
			if n%j == 0 {
				isPrime = true
				break
			}
		}
		if !isPrime {
			if min == -1 {
				min = i
				max = i
			} else {
				max = i
			}
		}
	}
	return max - min
}

func main() {
	fmt.Println(maximumPrimeDifference([]int{4, 2, 9, 5, 3}))
}
