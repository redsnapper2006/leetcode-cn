package main

import "fmt"

func isUgly(num int) bool {

	for num != 1 {
		if num%2 == 0 {
			num /= 2
			continue
		}
		if num%3 == 0 {
			num /= 3
			continue
		}
		if num%5 == 0 {
			num /= 5
			continue
		}
		return false
	}
	return true
}

func main() {
	fmt.Println("a")
}
