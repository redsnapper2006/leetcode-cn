package main

import (
	"fmt"
)

func lengthOfLastWord(s string) int {
	size := len(s)
	c := 0
	end := size - 1
	for i := size - 1; i >= 0; i-- {
		if s[i] == ' ' {
			end--
		} else {
			break
		}
	}
	for i := end; i >= 0; i-- {
		if (s[i] >= 'A' && s[i] <= 'Z') || (s[i] >= 'a' && s[i] <= 'z') {
			c++
		} else {
			break
		}
	}
	return c
}

func main() {
	fmt.Println("a")
}
