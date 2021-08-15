package main

import "fmt"

func kthFactor(n int, k int) int {
	if n == 1 {
		if k == 1 {
			return 1
		} else {
			return -1
		}
	}
	buf := make([]int, n+1)
	for i := 1; i*i <= n; i++ {
		if n%i == 0 {
			buf[i] = 1
			buf[n/i] = 1
		}
	}
	cnt := 0
	for i := 0; i < len(buf); i++ {
		if buf[i] == 1 {
			cnt++
			if cnt == k {
				return i
			}
		}
	}
	return -1
}

func main() {
	fmt.Println()
}
