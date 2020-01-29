package main

import (
	"fmt"
)

func isPalindrome(x int) bool {
	if x < 0 {
		return false
	}

	accum, t := 0, x
	for t/10 > 0 {
		accum = accum*10 + t%10
		t = t / 10
	}
	accum = accum*10 + t%10
	// fmt.Println(accum, x)
	if accum == x {
		return true
	}
	return false
}

func isPalindromeV2(x int) bool {
	if x < 0 {
		return false
	}

	var b []int
	for x/10 > 0 {
		b = append(b, x%10)
		x = x / 10
	}
	b = append(b, x%10)
	for i, j := 0, len(b)-1; i < j; i, j = i+1, j-1 {
		if b[i] != b[j] {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println(isPalindrome(1230))
	fmt.Println(isPalindrome(121))
	fmt.Println(isPalindrome(123321))
	fmt.Println(isPalindrome(4))
}
