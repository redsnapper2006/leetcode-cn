package main

import "fmt"

func selfDividingNumbers(left int, right int) []int {
	var buf []int
	for i := left; i < 10 && i <= right; i++ {
		buf = append(buf, i)
	}
	if left < 11 {
		left = 11
	}
	for i := left; i <= right; i++ {
		var digits []int
		n := i
		isHavingZero := false
		for n > 0 {
			if n%10 == 0 {
				isHavingZero = true
				break
			}
			digits = append(digits, n%10)
			n /= 10
		}
		if isHavingZero {
			continue
		}
		isSelf := true
		for _, d := range digits {
			if i%d != 0 {
				isSelf = false
				break
			}
		}
		if isSelf {
			buf = append(buf, i)
		}
	}
	return buf
}

func main() {
	fmt.Println("a")
}
