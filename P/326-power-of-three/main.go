package main

import (
	"fmt"
)

func isPowerOfThree(n int) bool {
	return n > 0 && 1162261467%n == 0
}

func isPowerOfThreeV2(n int) bool {
	if n == 0 {
		return false
	}
	for n >= 3 {
		if n%3 != 0 {
			return false
		} else {
			n = n / 3
		}
	}
	return n == 1
}

func main() {
	fmt.Println(isPowerOfThree(27))
}
