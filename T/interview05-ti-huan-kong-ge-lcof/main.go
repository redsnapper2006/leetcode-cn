package main

import "fmt"

func replaceSpace(s string) string {
	var buf []byte
	for i := 0; i < len(s); i++ {
		if s[i] == ' ' {
			buf = append(buf, '%', '2', '0')
		} else {
			buf = append(buf, s[i])
		}
	}
	return string(buf)
}

func main() {
	fmt.Println("a")
}
