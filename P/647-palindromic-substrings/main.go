package main

import "fmt"

func countSubstrings(s string) int {
	ret := len(s)

	for i := 1; i < len(s)-1; i++ {
		loop := i
		if loop > len(s)-i-1 {
			loop = len(s) - i - 1
		}
		for j := 1; j <= loop; j++ {
			if s[i-j] == s[i+j] {
				ret++
			} else {
				break
			}
		}
	}

	for i := 0; i < len(s)-1; i++ {
		loop := i
		if loop > len(s)-i-1-1 {
			loop = len(s) - i - 1 - 1
		}
		for j := 0; j <= loop; j++ {
			if s[i-j] == s[i+1+j] {
				ret++
			} else {
				break
			}
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
