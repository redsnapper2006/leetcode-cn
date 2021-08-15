package main

import "fmt"

func numOfStrings(patterns []string, word string) int {
	ret := 0
	for _, pat := range patterns {

		for i := 0; i <= len(word)-len(pat); i++ {
			isMatch := true
			if word[i] != pat[0] {
				continue
			}
			for j := 0; j < len(pat); j++ {
				if word[i+j] != pat[j] {
					isMatch = false
					break
				}
			}
			if isMatch {
				ret++
				break
			}
		}

	}
	return ret
}

func main() {
	fmt.Println()
}
