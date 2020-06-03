package main

import "fmt"

func reformat(s string) string {
	var l []byte
	var n []byte
	for i := 0; i < len(s); i++ {
		if s[i] <= '9' && s[i] >= '0' {
			n = append(n, s[i])
		} else {
			l = append(l, s[i])
		}
	}

	if len(l)-len(n) > 1 || len(n)-len(l) > 1 {
		return ""
	}
	m := l
	sub := n
	if len(l) < len(n) {
		m = n
		sub = l
	}
	var ret []byte
	i := 0
	for i < len(m) && i < len(sub) {
		ret = append(ret, m[i])
		ret = append(ret, sub[i])
		i++
	}
	if i < len(m) {
		ret = append(ret, m[i])
	}
	return string(ret)
}

func main() {
	fmt.Println("a")
}
