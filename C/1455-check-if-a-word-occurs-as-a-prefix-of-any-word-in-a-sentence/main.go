package main

import (
	"fmt"
	"strings"
)

func isPrefixOfWord(sentence string, searchWord string) int {
	arr := strings.Split(sentence, " ")
	for i := 0; i < len(arr); i++ {
		if strings.HasPrefix(arr[i], searchWord) {
			return i + 1
		}
	}
	return -1
}

func main() {
	fmt.Println("a")
}
