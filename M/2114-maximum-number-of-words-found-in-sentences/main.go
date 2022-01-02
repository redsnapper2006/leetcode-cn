package main

import "strings"

func mostWordsFound(sentences []string) int {
	max := -1
	for _, s := range sentences {
		c := len(strings.Split(s, " "))
		if c > max {
			max = c
		}
	}
	return max
}
