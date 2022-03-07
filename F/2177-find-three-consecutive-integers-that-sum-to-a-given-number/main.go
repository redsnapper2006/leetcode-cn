package main

import "fmt"

func sumOfThree(num int64) []int64 {
	if num%3 != 0 {
		return nil
	}
	n := num / 3
	return []int64{n - 1, n, n + 1}
}

func main() {
	fmt.Println()
}
