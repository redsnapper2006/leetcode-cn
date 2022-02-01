package main

import "fmt"

func divideString(s string, k int, fill byte) []string {
	c := 0
	ret := []string{}
	for c < len(s) {
		fc := c + k
		buf := []byte{}
		for c < fc && c < len(s) {
			buf = append(buf, s[c])
			c++
		}
		if c < fc {
			for c < fc {
				buf = append(buf, fill)
				c++
			}
		}
		ret = append(ret, string(buf))
	}
	return ret
}

func main() {
	fmt.Println()
}
