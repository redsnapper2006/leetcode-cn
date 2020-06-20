package main

import "fmt"

func waysToStep(n int) int {
	buf := make([]int, n+1)
	buf[0] = 1
	for i := 0; i <= n; i++ {
		if i+1 <= n {
			buf[i+1] += buf[i]
			buf[i+1] %= 1000000007
		}
		if i+2 <= n {
			buf[i+2] += buf[i]
			buf[i+2] %= 1000000007
		}
		if i+3 <= n {
			buf[i+3] += buf[i]
			buf[i+3] %= 1000000007
		}
	}
	return buf[n]
}

func main() {
	fmt.Println("a")
}
