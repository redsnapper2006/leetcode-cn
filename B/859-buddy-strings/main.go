package main

import "fmt"

func buddyStrings(A string, B string) bool {
	if len(A) != len(B) || len(A) < 2 {
		return false
	}

	c1, c2 := byte('0'), byte('0')
	times := 0
	for i, b := range A {
		if byte(b) == B[i] {
			continue
		}
		times++
		if times > 2 {
			return false
		}
		if times == 2 {
			if !(c1 == B[i] && c2 == byte(b)) {
				return false
			}
		}
		c1 = byte(b)
		c2 = B[i]
	}
	if times == 1 {
		return false
	}
	if times == 0 {
		b := make([]int, 26)
		for _, v := range A {
			b[v-'a']++
		}

		for _, bb := range b {
			if bb > 1 {
				return true
			}
		}
		return false
	}
	return true
}

func main() {
	fmt.Println("a")
}
