package main

import (
	"fmt"
	"strings"
)

func truncateSentence(s string, k int) string {
	return strings.Join(strings.Split(s, " ")[0:k], " ")
}

func main() {
	fmt.Println(truncateSentence("hello world", 2))
}
