package main

import "fmt"

func countValidWords(sentence string) int {
	count := 0
	isValid := true
	hyphonCnt := 0
	letterCnt := 0
	symbolCnt := 0
	for i, b := range sentence {
		if byte(b) == ' ' {
			// fmt.Println(i, letterCnt, symbolCnt, isValid)
			if (letterCnt > 0 || symbolCnt == 1) && isValid {
				// fmt.Println(i, string(b))
				count++
			}
			isValid = true
			hyphonCnt = 0
			letterCnt = 0
			symbolCnt = 0
			continue
		}
		if byte(b) >= byte('a') && byte(b) <= byte('z') {
			letterCnt++
		}
		if byte(b) >= byte('0') && byte(b) <= byte('9') {
			isValid = false
		}
		if byte(b) == byte('-') {
			hyphonCnt++
			if hyphonCnt > 1 || i == 0 || i == len(sentence)-1 || !(byte(sentence[i-1]) >= byte('a') && byte(sentence[i-1]) <= byte('z') &&
				byte(sentence[i+1]) >= byte('a') && byte(sentence[i+1]) <= byte('z')) {
				isValid = false
			}
		}
		if byte(b) == byte('!') || byte(b) == byte(',') || byte(b) == byte('.') {
			symbolCnt++
			// fmt.Println(i)
			if symbolCnt > 1 || !((i == 0 && (len(sentence) == 1 || sentence[i+1] == byte(' '))) ||
				(i == len(sentence)-1 && byte(sentence[i-1]) >= byte('a') && byte(sentence[i-1]) <= byte('z')) || (i > 0 && i < len(sentence)-1 && (byte(sentence[i-1]) >= byte('a') && byte(sentence[i-1]) <= byte('z') || byte(sentence[i-1]) == byte(' ')) &&
				byte(sentence[i+1]) >= byte(' ') && byte(sentence[i+1]) <= byte(' '))) {
				isValid = false
			}
		}
	}
	if (letterCnt > 0 || symbolCnt == 1) && isValid {
		count++
	}
	return count
}

func main() {
	fmt.Println()
}
