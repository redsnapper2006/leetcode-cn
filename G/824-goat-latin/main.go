package main

import (
	"fmt"
	"strings"
)

func toGoatLatin(S string) string {
	arr := strings.Split(S, " ")
	var buf []string
	suffix := ""
	for i := 0; i < len(arr); i++ {
		b := arr[i]
		suffix += "a"
		if b[0] == 'A' || b[0] == 'a' || b[0] == 'E' || b[0] == 'e' || b[0] == 'I' || b[0] == 'i' || b[0] == 'O' || b[0] == 'o' || b[0] == 'U' || b[0] == 'u' {
			buf = append(buf, b+"ma"+suffix)
		} else {
			initial := b[0]
			buf = append(buf, b[1:]+string(initial)+"ma"+suffix)
		}
	}
	return strings.Join(buf, " ")
}

func main() {
	fmt.Println("a")
}
