package main

import (
	"fmt"
)

func firstUniqChar(s string) int {
	var letters [26]int
	for i, c := range s {
		// fmt.Println(c - 'a')
		if letters[int(c-'a')] > 0 {
			letters[int(c-'a')] = len(s) + 1
		} else {
			letters[int(c-'a')] = i + 1
		}
	}
	r := len(s) + 1
	for i := 0; i < 26; i++ {
		if letters[i] > 0 && letters[i] < len(s)+1 {
			if letters[i] < r {
				r = letters[i]
			}
		}
	}
	if r == len(s)+1 {
		return -1
	} else {
		return r - 1
	}
}

func main() {
	fmt.Println("h")
	fmt.Println(firstUniqChar("loveleetcode"))
}
