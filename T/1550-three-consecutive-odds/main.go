package main

import "fmt"

func threeConsecutiveOdds(arr []int) bool {
	c := 0
	for _, v := range arr {
		if v%2 == 1 {
			c++
			if c == 3 {
				return true
			}
		} else {
			c = 0
		}
	}
	return false
}

func main() {
	fmt.Println("a")
}
