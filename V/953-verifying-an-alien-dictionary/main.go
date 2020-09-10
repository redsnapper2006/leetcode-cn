package main

import "fmt"

func isAlienSorted(words []string, order string) bool {
	maxLen := -1

	for _, v := range words {
		if len(v) > maxLen {
			maxLen = len(v)
		}
	}
	dict := make([]int, 26)
	for i, b := range order {
		dict[b-'a'] = i + 1
	}
	buf := make([][]int, len(words))
	for i, w := range words {
		buf[i] = make([]int, maxLen)
		for j, b := range w {
			buf[i][j] = dict[b-'a']
		}
	}

	for i := 0; i < len(words)-1; i++ {
		isValid := true
		for j := 0; j < maxLen; j++ {
			if buf[i][j] > buf[i+1][j] {
				isValid = false
				break
			} else if buf[i][j] < buf[i+1][j] {
				break
			}
		}
		if !isValid {
			return false
		}
	}

	return true
}

func main() {
	fmt.Println("a")
}
