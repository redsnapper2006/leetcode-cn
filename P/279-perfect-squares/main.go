package main

import (
	"fmt"
)

func numSquares(n int) int {
	buf := make([]int, n+1)
	buf[n] = 1
	for i := n; i >= 1; i-- {
		a := buf[i]

		for j := 1; j*j <= i; j++ {

			aj := buf[i-j*j]
			if aj == 0 {
				buf[i-j*j] = a + 1
			} else {
				if aj > a+1 {
					buf[i-j*j] = a + 1
				}
			}
		}
	}
	return buf[0] - 1
}

func main() {
	fmt.Println(numSquares(12))
	fmt.Println(numSquares(13))
	fmt.Println(numSquares(25))
}
