package main

import "fmt"

func oneEditAway(first string, second string) bool {
	if first == second {
		return true
	}
	if len(first)-len(second) > 1 || len(second)-len(first) > 1 {
		return false
	}

	b, l := first, second
	if len(first) < len(second) {
		b, l = second, first
	}

	lIdx := 0
	firstUnmatch := false
	for i := 0; i < len(b)&& lIdx < len(l); i++ {
		if b[i] == l[lIdx] {
			lIdx++
			continue
		}
		if firstUnmatch == false {
			firstUnmatch = true
			if len(b) == len(l) {
				lIdx++
			}
		} else {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println("a")
}
