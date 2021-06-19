package main

import "fmt"

func leastMinutes(n int) int {
	count := 0
	for n > 1 {
		// fmt.Println(n)
		t := n / 2
		if n%2 > 0 {
			t++
		}
		count++
		n = t
	}
	return count + 1
}

func main() {
	fmt.Println(leastMinutes(2))
	fmt.Println(leastMinutes(4))
}
