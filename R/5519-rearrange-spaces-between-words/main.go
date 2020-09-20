package main

import (
	"fmt"
	"strings"
)

func reorderSpaces(text string) string {
	spaces := 0
	for _, b := range text {
		if b == ' ' {
			spaces++
		}
	}
	words := 0
	wordArr := []string{}
	for _, v := range strings.Split(strings.TrimSpace(text), " ") {
		if v != "" {
			words++
			wordArr = append(wordArr, v)
		}
	}
	scnt, sremain := 0, 0
	if words == 1 {
		sremain = spaces
	} else {
		scnt = spaces / (words - 1)
		sremain = spaces % (words - 1)
	}
	scandi := ""
	for i := 0; i < scnt; i++ {
		scandi += " "
	}

	ret := wordArr[0]
	for i := 0; i < (words - 1); i++ {
		ret += scandi + wordArr[i+1]
	}
	for i := 0; i < sremain; i++ {
		ret += " "
	}
	return ret
}

func main() {
	fmt.Println(reorderSpaces("  this   is  a sentence "))
}
