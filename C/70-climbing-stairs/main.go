package main

import "fmt"

func climbStairs(n int) int {
	if n <= 2 {
		return n
	}
	b := make([]int, n)
	b[0] = 1
	b[1] = 2
	for i := 2; i < n; i++ {
		b[i] = b[i-1] + b[i-2]
	}
	return b[n-1]
}

func main() {
	fmt.Println("a")
}
