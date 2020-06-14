package main

import "fmt"

func replaceSpaces(S string, length int) string {
	var buf []byte
	for i := 0; i < length; i++ {
		if S[i] == ' ' {
			buf = append(buf, '%', '2', '0')
		} else {
			buf = append(buf, S[i])
		}
	}
	return string(buf)
}

func main() {
	fmt.Println("a")
}
