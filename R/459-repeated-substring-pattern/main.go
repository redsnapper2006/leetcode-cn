package main

import "fmt"

func repeatedSubstringPattern(s string) bool {
	size := len(s) / 2
	for i := size; i > 0; i++ {
		if len(s)%i != 0 {
			continue
		}
		base := s[0:i]
		t := ""
		for j := 0; j < len(s)/i; j++ {
			t += base
		}
		if t == s {
			return true
		}
	}
	return false
}

func main() {
	fmt.Println("a")
}
