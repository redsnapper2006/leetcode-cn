package main

import "fmt"

func firstPalindrome(words []string) string {
	for _, w := range words {
		s, e := 0, len(w)-1
		isPal := true
		for s < e {
			if byte(w[s]) != byte(w[e]) {
				isPal = false
				break
			}
			s++
			e--
		}
		if isPal {
			return w
		}
	}
	return ""
}

func main() {
	fmt.Println()
}
