package main

import "fmt"

func backspaceCompare(S string, T string) bool {
	trim := func(s string) string {
		var buf []byte
		for i := 0; i < len(s); i++ {
			if s[i] != '#' {
				buf = append(buf, s[i])
			} else if len(buf) > 0 {
				buf = buf[0 : len(buf)-1]
			}
		}
		return string(buf)
	}
	return trim(S) == trim(T)
}

func main() {
	fmt.Println("a")
}
