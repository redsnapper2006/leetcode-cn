package main

import "fmt"

func validPalindrome(s string) bool {
	b, e := 0, len(s)-1
	for b < e {
		if s[b] != s[e] {
			break
		}
		b++
		e--
	}
	if b >= e {
		return true
	}

	valid := func(b, e int, s string) bool {
		for b < e {
			if s[b] != s[e] {
				break
			}
			b++
			e--
		}
		if b >= e {
			return true
		}
		return false
	}

	return valid(b+1, e, s) || valid(b, e-1, s)
}

func main() {
	fmt.Println(validPalindrome("aguokepatgbnvfqmgmlcupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupuculmgmqfvnbgtapekouga"))
}
