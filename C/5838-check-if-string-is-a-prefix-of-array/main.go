package main

import "fmt"

func isPrefixString(s string, words []string) bool {
	idx := 0
	for i := 0; i < len(words); i++ {
		for j := 0; j < len(words[i]); j++ {
			if s[idx] == words[i][j] {
				idx++
				if idx == len(s) && j < len(words[i])-1 {

					return false
				}
			} else {
				return false
			}
		}
		if idx == len(s) {
			return true
		}
	}
	return idx == len(s)
}

func main() {
	fmt.Println()
}
