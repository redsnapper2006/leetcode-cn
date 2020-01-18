package main

import (
	"fmt"
)

func isAnagram(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}

	var sc, tc [26]int
	for _, c := range s {
		sc[c-'a']++
	}
	for _, c := range t {
		tc[c-'a']++
	}
	for i := 0; i < 26; i++ {
		if sc[i] != tc[i] {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println(isAnagram("rat", "car"))
}
