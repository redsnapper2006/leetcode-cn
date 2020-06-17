package main

import "fmt"

func canPermutePalindrome(s string) bool {
	M := map[byte]int{}
	for i := 0; i < len(s); i++ {
		M[s[i]]++
	}
	firstOdd := false
	for _, v := range M {
		if v%2 == 1 {
			if !firstOdd {
				firstOdd = true
			} else {
				return false
			}
		}
	}
	return true
}

func main() {
	fmt.Println("a")
}
