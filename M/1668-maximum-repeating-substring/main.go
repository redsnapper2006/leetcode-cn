package main

import "fmt"

func maxRepeating(sequence string, word string) int {
	if len(sequence) < len(word) {
		return 0
	}

	idx := 0
	ret := 0
	for idx+len(word) <= len(sequence) {
		if sequence[idx:idx+len(word)] == word {
			cnt := 0
			i := idx
			for i+len(word) <= len(sequence) && sequence[i:i+len(word)] == word {
				cnt++
				i += len(word)
			}
			if cnt > ret {
				ret = cnt
			}
		}

		idx++
	}

	return ret
}

func main() {
	fmt.Println()
}
