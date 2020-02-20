package main

import (
	"fmt"
)

func strStr(haystack string, needle string) int {
	if len(needle) == 0 {
		return 0
	}

	for i := 0; i < len(haystack)-len(needle)+1; i++ {
		if haystack[i] == needle[0] {
			isMatch := true
			for j := 1; j < len(needle); j++ {
				if haystack[i+j] != needle[j] {
					isMatch = false
					break
				}
			}
			if isMatch {
				return i
			}
		}
	}
	return -1
}

func main() {
	fmt.Println(strStr("hello", "ll"))
}
