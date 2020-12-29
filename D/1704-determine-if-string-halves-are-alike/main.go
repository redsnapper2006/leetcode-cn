package main

import "fmt"

func halvesAreAlike(s string) bool {
	left, right := 0, 0
	for i := 0; i < len(s)/2; i++ {
		if byte(s[i]) == 'a' || byte(s[i]) == 'e' || byte(s[i]) == 'i' || byte(s[i]) == 'o' || byte(s[i]) == 'u' || byte(s[i]) == 'A' || byte(s[i]) == 'E' || byte(s[i]) == 'I' || byte(s[i]) == 'O' || byte(s[i]) == 'U' {
			left++
		}
		if byte(s[len(s)-1-i]) == 'a' || byte(s[len(s)-1-i]) == 'e' || byte(s[len(s)-1-i]) == 'i' || byte(s[len(s)-1-i]) == 'o' || byte(s[len(s)-1-i]) == 'u' || byte(s[len(s)-1-i]) == 'A' || byte(s[len(s)-1-i]) == 'E' || byte(s[len(s)-1-i]) == 'I' || byte(s[len(s)-1-i]) == 'O' || byte(s[len(s)-1-i]) == 'U' {
			right++
		}
	}
	return left == right
}

func main() {
	fmt.Println()
}
