package main

import (
	"fmt"
	"strconv"
	"strings"
)

func compressString(S string) string {
	if len(S) <= 2 {
		return S
	}

	r := []string{}
	s := S[0]
	c := 1
	for i := 1; i < len(S); i++ {
		if S[i] == s {
			c++
		} else {
			r = append(r, string([]byte{s})+strconv.FormatInt(int64(c), 10))
			s = S[i]
			c = 1
		}
	}
	r = append(r, string([]byte{s})+strconv.FormatInt(int64(c), 10))
	ret := strings.Join(r, "")
	if len(ret) > len(S) {
		return S
	}
	return ret
}

func main() {
	fmt.Println("a")
}
