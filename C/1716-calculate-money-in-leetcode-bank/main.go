package main

import "fmt"

func totalMoney(n int) int {
	sum := 0
	start := 0
	for i := 0; i < n; i++ {
		if i%7 == 0 {
			start++
		}
		sum += start + i%7
	}
	return sum
}

func main() {
	fmt.Println()
}
