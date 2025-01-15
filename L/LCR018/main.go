package main

import "fmt"

func isPalindrome(s string) bool {
	idxs, idxe := 0, len(s)-1
	for idxs < idxe {
		if !((s[idxs] >= 'a' && s[idxs] <= 'z') || (s[idxs] >= 'A' && s[idxs] <= 'Z') || (s[idxs] >= '0' && s[idxs] <= '9')) {
			idxs++
			continue
		}
		if !((s[idxe] >= 'a' && s[idxe] <= 'z') || (s[idxe] >= 'A' && s[idxe] <= 'Z') || (s[idxe] >= '0' && s[idxe] <= '9')) {
			idxe--
			continue
		}
		sc := s[idxs]
		if s[idxs] >= 'A' && s[idxs] <= 'Z' {
			sc = byte('a' + int(s[idxs]-'A'))
		}
		ec := s[idxe]
		if s[idxe] >= 'A' && s[idxe] <= 'Z' {
			ec = byte('a' + int(s[idxe]-'A'))
		}
		if sc != ec {
			return false
		}
		idxs++
		idxe--
	}
	return true
}

func main() {
	fmt.Println()
}
