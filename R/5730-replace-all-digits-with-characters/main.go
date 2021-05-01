package main

import "fmt"

func replaceDigits(s string) string {
	buf := make([]byte, len(s))
	b := byte('0')
	for i := 0; i < len(s); i++ {
		if i%2 == 0 {
			b = byte(s[i])
			buf[i] = b
		} else {
			buf[i] = byte(int(b) + int(s[i]-'0'))
		}
	}
	return string(buf)
}

func main() {
	fmt.Println(replaceDigits("a1c1e1"))
}
