package main

import (
	"fmt"
	"strings"
)

func isValidSerialization(preorder string) bool {
	arr := strings.Split(preorder, ",")

	var recur func(arr []string) ([]string, bool)
	recur = func(arr []string) ([]string, bool) {
		if len(arr) == 0 {
			return nil, false
		}
		if arr[0] == "#" {
			return arr[1:], true
		}
		// left
		left, lb := recur(arr[1:])
		if !lb || len(left) == 0 {
			return nil, false
		}
		return recur(left)
	}
	a, b := recur(arr)
	return b && len(a) == 0
}

func main() {
	fmt.Println("a")
}
