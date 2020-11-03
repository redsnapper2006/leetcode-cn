package main

import "fmt"

func swapNumbers(numbers []int) []int {
	numbers = append(numbers, numbers[0])
	return numbers[1:]
}

func main() {
	fmt.Println()
}
