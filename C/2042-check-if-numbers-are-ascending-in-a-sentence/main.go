package main

import "fmt"

func areNumbersAscending(s string) bool {
	prev, curr := 0, 0
	for _, b := range s {
		if byte(b) >= byte('0') && byte(b) <= byte('9') {
			curr = curr*10 + int(byte(b)-byte('0'))
		} else {
			if prev != 0 && curr != 0 {
				if prev >= curr {
					return false
				}
			}
			if curr > 0 {
				prev = curr
			}
			curr = 0
		}
	}

	if prev != 0 && curr != 0 {
		if prev >= curr {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println()
}
