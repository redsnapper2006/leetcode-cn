package main

import "fmt"

func lastRemaining(n int) int {
	a1 := 1
	k, cnt, step := 0, n, 1
	for cnt > 1 {
		if k%2 == 0 || cnt%2 != 0 {
			a1 += step
		}
		k++
		step <<= 1
		cnt >>= 1
	}
	return a1
}

func main() {
	fmt.Println()
}
