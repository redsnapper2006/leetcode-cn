package main

import (
	"fmt"
	"strings"
)

func wordPattern(pattern string, str string) bool {
	wordList := strings.Split(str, " ")

	if len(pattern) != len(wordList) {
		return false
	}

	M1 := make(map[byte]string)
	M2 := make(map[string]byte)
	for i := 0; i < len(pattern); i++ {
		_, ok := M1[pattern[i]]
		if !ok {
			M1[pattern[i]] = wordList[i]
			_, ok2 := M2[wordList[i]]
			if !ok2 {
				M2[wordList[i]] = pattern[i]
			} else {
				return false
			}
		} else {
			if M1[pattern[i]] != wordList[i] {
				return false
			}
		}
	}
	return true
}

func main() {
	fmt.Println("a")
}
