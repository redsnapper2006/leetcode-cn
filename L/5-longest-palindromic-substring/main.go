package main

import (
	"fmt"
)

func longestPalindrome(s string) string {
	if len(s) == 1 {
		return s
	}

	size := 2*len(s) + 1
	ret := ""

	for i := 2; i <= size-3; i++ {
		var s1, s2 int
		if i%2 == 0 {
			s2 = i / 2
			s1 = s2 - 1
		} else {
			s2 = i/2 + 1
			s1 = s2 - 2
		}
		if s[s1] != s[s2] {

			if len(ret) < 1 {
				ret = s[i/2 : i/2+1]
			}

			continue
		}
		for s1 >= 0 && s2 < len(s) {
			if s[s1] == s[s2] {
				s1--
				s2++
			} else {
				break
			}
		}
		s1++
		s2--

		if len(ret) < s2-s1+1 {
			ret = s[s1 : s2+1]
		}
	}

	return ret
}

func main() {
	fmt.Println(longestPalindrome("ac"))
}
