package main

import (
	"fmt"
)

func checkInclusion(s1 string, s2 string) bool {
	if len(s2) < len(s1) {
		return false
	}
	isMatch := func(B1, B2 []int) bool {
		for i := 0; i < len(B1); i++ {
			if B1[i] != B2[i] {
				return false
			}
		}
		return true
	}

	B1 := make([]int, 26)
	for i := 0; i < len(s1); i++ {
		B1[s1[i]-'a']++
	}

	B2 := make([]int, 26)
	for i := 0; i < len(s1); i++ {
		B2[s2[i]-'a']++
	}
	if isMatch(B1, B2) {
		return true
	}

	for i := 1; i < len(s2)-len(s1)+1; i++ {
		B2[s2[i-1]-'a']--
		B2[s2[i+len(s1)-1]-'a']++
		if isMatch(B1, B2) {
			return true
		}
	}
	return false
}

func main() {
	fmt.Println(checkInclusion("ab", "eidbaooo"))
}
