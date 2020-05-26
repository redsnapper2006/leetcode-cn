package main

import (
	"fmt"
	"strings"
)

func findOcurrences(text string, first string, second string) []string {
	var ret []string
	wordArr := strings.Split(text, " ")
	for i := 0; i < len(wordArr); i++ {
		if i < len(wordArr)-2 && wordArr[i] == first && wordArr[i+1] == second {
			ret = append(ret, wordArr[i+2])
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
