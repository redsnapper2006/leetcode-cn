package main

import "fmt"

func reverseLeftWords(s string, n int) string {
	buf := make([]byte, len(s))
	for i := 0; i < len(s); i++ {
		buf[(i-n+len(s))%len(s)] = s[i]
	}
	return string(buf)
}

func main() {
	fmt.Println("a")
}
