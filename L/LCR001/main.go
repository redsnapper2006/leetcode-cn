package main

import "fmt"

func divide(a int, b int) int {
	if a == 0 {
		return 0
	}

	if a == 1<<31-1 {
		if b == 1 {
			return 1<<31 - 1
		} else if b == -1 {
			return -1<<31 + 1
		}
	}
	if a == -1<<31 {
		if b == 1 {
			return -1 << 31
		} else if b == -1 {
			return 1<<31 - 1
		}
	}

	isMinus := false
	if (a > 0 && b < 0) || (a < 0 && b > 0) {
		isMinus = true
	}

	ABS := func(n int) int {
		y := n >> 31
		return (n ^ y) - y
	}

	if a < 0 {
		a = ABS(a)
	}
	if b < 0 {
		b = ABS(b)
	}

	times := 0
	extra := 0
	for i := 31; i >= 0; i-- {
		if (extra + (b << i)) <= a {
			extra += (b << i)
			times |= 1 << i
		}
	}

	if isMinus {
		times = -times
	}

	return times
}

func main() {
	fmt.Println()
}
