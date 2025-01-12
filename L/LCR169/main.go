package main

import "fmt"

func firstUniqChar(s string) byte {
	M := map[byte]int{}
	for i := 0; i < len(s); i++ {
		M[s[i]]++
	}
	for i := 0; i < len(s); i++ {
		if M[s[i]] == 1 {
			return s[i]
		}
	}
	return ' '
}

func main() {
	fmt.Println("a")
}
