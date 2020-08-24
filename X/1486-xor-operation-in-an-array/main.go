package main

import "fmt"

func xorOperation(n int, start int) int {
	xor := start

	for i := 1; i < n; i++ {
		xor ^= start + 2*i
	}

	return xor
}

func main() {
	fmt.Println("a")
}
