package main

import "fmt"

func removePalindromeSub(s string) int {
	buf := []byte(s)
	ss, e := 0, len(s)-1
	for ss < e {
		if buf[ss] != buf[e] {
			return 2
		}
		ss++
		e--
	}
	return 1
}

func main() {
	fmt.Println()
}
