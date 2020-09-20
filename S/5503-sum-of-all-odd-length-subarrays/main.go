package main

import "fmt"

func sumOddLengthSubarrays(arr []int) int {
	sum := 0
	buf := make([]int, len(arr))
	for i, v := range arr {
		sum += v
		buf[i] = sum
	}

	accum := 0
	accum += sum
	for i := 2; i < len(arr); i = i + 2 {
		accum += buf[i]
		for j := i + 1; j < len(arr); j++ {
			accum += buf[j] - buf[j-i-1]
		}
	}
	return accum
}

func main() {
	fmt.Println("a")
}
