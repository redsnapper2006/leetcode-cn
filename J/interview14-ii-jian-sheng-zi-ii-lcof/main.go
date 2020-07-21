package main

import "fmt"

func cuttingRope(n int) int {

	buf := make([]int, n+1)

	buf[1] = 1

	for i := 2; i <= n; i++ {
		max := -1
		for j := 1; j <= i/2; j++ {
			r := j * (i - j)
			if r < buf[j]*(i-j) {
				r = buf[j] * (i - j)
			}
			if r < j*buf[i-j] {
				r = j * buf[i-j]
			}
			if r < buf[j]*buf[i-j] {
				r = buf[j] * buf[i-j]
			}
			if r > max {
				max = r
			}
		}
		buf[i] = max
	}
	return buf[n]
}

func main() {
	fmt.Println("a")
}
