package main

import "fmt"

func maxRepeating(sequence string, word string) int {
	if len(sequence) < len(word) {
		return 0
	}
	idx := 0
	ret := 0
	for idx < len(sequence) {
		for idx < len(sequence) && sequence[idx] != word[0] {
			idx++
		}
		if idx == len(sequence) {
			break
		}
		loop := 0
		for idx+len(word) <= len(sequence) {
			isMatch := true
			for i := 0; i < len(word); i++ {
				if word[i] != sequence[idx+i] {
					isMatch = false
					break
				}
			}
			if isMatch {
				loop++
				idx += len(word)
			} else {
				break
			}
		}
		idx++
		if loop > ret {
			ret = loop
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
