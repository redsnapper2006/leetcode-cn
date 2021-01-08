package main

import "fmt"

func breakPalindrome(palindrome string) string {
	buf := []byte(palindrome)
	size := len(buf) / 2
	isReplace := false
	for i := 0; i < size; i++ {
		if buf[i] != 'a' {
			buf[i] = 'a'
			isReplace = true
			break
		}
	}
	if isReplace {
		return string(buf)
	}
	if len(buf) == 1 {
		return ""
	}
	buf[len(buf)-1] = 'b'
	return string(buf)
}

func main() {
	fmt.Println()
}
