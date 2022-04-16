package main

import "math"

func largestPalindrome(n int) int {

	// return []int{0, 9, 987, 123, 597, 677, 1218, 877, 475}[n]

	if n == 1 {
		return 9
	}
	upper := int(math.Pow10(n)) - 1
	for left := upper; ; left-- {
		p := left
		for x := left; x > 0; x /= 10 {
			p = p*10 + x%10
		}
		for x := upper; x*x >= p; x-- {
			if p%x == 0 {
				return p % 1337
			}
		}
	}
}
