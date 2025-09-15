package main

import (
	"fmt"
	"strings"
)

func canBeTypedWords(text string, brokenLetters string) int {
	broken := make([]int, 26)

	for _, b := range brokenLetters {
		broken[int(byte(b)-byte('a'))] = 1
	}

	arr := strings.Split(text, " ")
	ret := 0
	for _, t := range arr {
		canBeInput := true
		for _, b := range t {
			if broken[int(byte(b)-byte('a'))] == 1 {
				canBeInput = false
				break
			}
		}
		if canBeInput {
			ret++
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
