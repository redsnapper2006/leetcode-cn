package main

import "fmt"

func getMaximumGenerated(n int) int {
	if n < 2 {
		return n
	}
	buf := make([]int, n+1)
	buf[0] = 0
	buf[1] = 1
	for i := 2; i < n+1; i++ {
		if i%2 == 0 {
			buf[i] = buf[i/2]
		} else {
			buf[i] = buf[i/2] + buf[i/2+1]
		}
	}
	max := -1
	for i := 0; i < n+1; i++ {
		if max < buf[i] {
			max = buf[i]
		}
	}
	return max
}

func main() {
	fmt.Println()
}
