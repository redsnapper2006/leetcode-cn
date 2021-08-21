package main

import "fmt"

func minTimeToType(word string) int {
	cur := 'a'
	steps := 0
	for _, b := range word {
		offset := int(b - cur)
		if offset < 0 {
			offset = -offset
		}
		if offset > 26-offset {
			offset = 26 - offset
		}

		steps += offset
		steps++
		cur = b
	}
	return steps
}

func main() {
	fmt.Println()
}
