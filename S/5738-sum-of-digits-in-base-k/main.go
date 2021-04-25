package main

import "fmt"

func sumBase(n int, k int) int {
	sum := 0

	for n > 0 {
		sum += n % k
		n /= k
	}
	return sum
}

func main() {
	fmt.Println(sumBase(34, 6))
}
