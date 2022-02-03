package main

import "fmt"

func findMinFibonacciNumbers(k int) int {
	fib := []int{1, 1}
	x, y := 1, 1
	for x < k && y < k {
		x, y = x+y, x
		fib = append(fib, x)
	}

	steps := 0
	t := k
	idx := len(fib) - 1
	for t > 0 {
		if t >= fib[idx] {
			t -= fib[idx]
			steps++
		}
		idx--
	}
	return steps
}

func main() {
	fmt.Println(findMinFibonacciNumbers(5799990))
	fmt.Println(findMinFibonacciNumbers(8318449))
	fmt.Println(findMinFibonacciNumbers(645157245))
}
