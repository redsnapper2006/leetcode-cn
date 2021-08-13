package main

import "fmt"

func isAnagram(s string, t string) bool {
	if len(s) != len(t) || s == t {
		return false
	}

	bufS, bufT := make([]int, 26), make([]int, 26)
	for i := 0; i < len(s); i++ {
		bufS[int(s[i]-'a')]++
		bufT[int(t[i]-'a')]++
	}

	for i := 0; i < 26; i++ {
		if bufS[i] != bufT[i] {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println()
}
