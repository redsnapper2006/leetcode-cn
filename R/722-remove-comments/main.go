package main

import (
	"fmt"
)

func removeComments(source []string) []string {
	var ret []string
	isBlockComm := false
	var buf []byte
	for i := 0; i < len(source); i++ {
		s := source[i]
		j := 0
		for j < len(s) {
			if !isBlockComm && j+1 < len(s) && s[j] == '/' && s[j+1] == '*' {
				isBlockComm = true
				j += 2
				continue
			}
			if !isBlockComm && j+1 < len(s) && s[j] == '/' && s[j+1] == '/' {
				break
			}
			if isBlockComm && j+1 < len(s) && s[j] == '*' && s[j+1] == '/' {
				isBlockComm = false
				j += 2
				continue
			}
			if !isBlockComm {
				buf = append(buf, s[j])
			}
			j++
		}
		if len(buf) == 0 {
			continue
		}
		if !isBlockComm {
			ret = append(ret, string(buf))
			buf = []byte{}
		}
	}
	return ret
}

func main() {
	fmt.Println(removeComments([]string{"a/*comment", "line", "more_comment*/b"}))
}
