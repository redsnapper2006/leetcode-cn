package main

import "fmt"

func checkZeroOnes(s string) bool {
	base := byte('1')
	if s[0] == byte('1') {
		base = byte('0')
	}
	one, zero := 0, 0
	oneMax, zeroMax := -1, -1
	for i := 0; i < len(s); i++ {
		if s[i] != base {
			if base == byte('0') && zero > zeroMax {
				zeroMax = zero
			}
			if base == byte('1') && one > oneMax {
				oneMax = one
			}
			base = s[i]
			if base == byte('0') {
				zero = 1
				one = 0
			}
			if base == byte('1') {
				one = 1
				zero = 0
			}
		} else {
			if base == byte('0') {
				zero++
			}
			if base == byte('1') {
				one++
			}
		}
	}
	if base == byte('0') && zero > zeroMax {
		zeroMax = zero
	}
	if base == byte('1') && one > oneMax {
		oneMax = one
	}
	return oneMax > zeroMax
}

func main() {
	fmt.Println()
}
