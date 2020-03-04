package main

import "fmt"

func isPowerOfTwo(n int) bool {
	t := 0

	for {
		b := 1 << t
		if b > n {
			return false
		} else if b == n {
			return true
		}
		t++
		if t > 64 {
			break
		}
	}
	return false
}

func main() {
	fmt.Println("a")
}
