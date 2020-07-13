package main

import "fmt"

func reverseStr(s string, k int) string {
	var buf []byte
	for i := 0; i < len(s); i = i + 2*k {
		e := i + k - 1
		if e > len(s)-1 {
			e = len(s) - 1
		}

		for j := e; j >= i; j-- {
			buf = append(buf, s[j])
		}
		for j := 0; j < k && e+1+j < len(s); j++ {
			buf = append(buf, s[e+1+j])
		}
	}
	return string(buf)
}

func main() {
	fmt.Println("a")
}
