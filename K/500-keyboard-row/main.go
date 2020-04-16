package main

import (
	"fmt"
	"strings"
)

func findWords(words []string) []string {
	M := map[byte]int{
		'q': 1, 'w': 1, 'e': 1, 'r': 1, 't': 1, 'y': 1, 'u': 1, 'i': 1, 'o': 1, 'p': 1,
		'a': 2, 's': 2, 'd': 2, 'f': 2, 'g': 2, 'h': 2, 'j': 2, 'k': 2, 'l': 2,
		'z': 3, 'x': 3, 'c': 3, 'v': 3, 'b': 3, 'n': 3, 'm': 3,
	}
	var buf []string
	for i := 0; i < len(words); i++ {
		w := strings.ToLower(words[i])
		lineNo := M[w[0]]
		isSameLine := true
		for j := 1; j < len(w); j++ {
			if M[w[j]] != lineNo {
				isSameLine = false
				break
			}
		}
		if isSameLine {
			buf = append(buf, words[i])
		}
	}
	return buf
}

func main() {
	fmt.Println("a")
}
