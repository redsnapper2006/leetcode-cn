package main

import "fmt"

func rand7() int {
	return 1
}

func rand10() int {
	for {
		r := rand7()
		c := rand7()
		idx := c + (r-1)*7
		if idx <= 40 {
			return 1 + (idx-1)%10
		}
	}
}

func main() {
	fmt.Println()
}
