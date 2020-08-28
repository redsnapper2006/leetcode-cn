package main

import "fmt"

func maxPower(s string) int {
	b := s[0]
	c := 1
	max := 1
	for i := 1; i < len(s); i++ {
		if s[i] == b {
			c++
		} else {
			if c > max {
				max = c
			}
			b = s[i]
			c = 1
		}
	}
	if c > max {
		max = c
	}
	return max
}

func main() {
	fmt.Println("a")
}
