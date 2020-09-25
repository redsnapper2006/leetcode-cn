package main

import "fmt"

func countLargestGroup(n int) int {
	M := map[int]int{}

	for i := 1; i <= n; i++ {
		t := i
		c := 0
		for t > 0 {
			c += t % 10
			t /= 10
		}
		M[c]++
	}
	max := -1
	count := -1

	for _, v := range M {
		if v > max {
			max = v
			count = 1
		} else if v == max {
			count++
		}
	}
	return count
}

func main() {
	fmt.Println("a")
}
