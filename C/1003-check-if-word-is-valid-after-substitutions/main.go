package main

import (
	"fmt"
	"strings"
)

func isValid(S string) bool {
	b := S

	for {
		r := strings.Replace(b, "abc", "", -1)
		if r == b {
			return false
		}
		if r == "" {
			return true
		}
		b = r
	}
}

func main() {
	fmt.Println("a")
}
