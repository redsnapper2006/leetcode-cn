package main

import "fmt"

func toLowerCase(str string) string {
	buf := make([]byte, len(str))
	for i := 0; i < len(str); i++ {
		if str[i] <= 'Z' && str[i] >= 'A' {
			buf[i] = byte(str[i] - 'A' + 'a')
		} else {
			buf[i] = str[i]
		}
	}
	return string(buf)
}

func main() {
	fmt.Println("a")
}
