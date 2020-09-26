package main

import "fmt"

func freqAlphabets(s string) string {
	idx := len(s) - 1
	var buf []byte
	for idx >= 0 {
		if s[idx] == '#' {
			buf = append(buf, byte(int(s[idx-2]-'0')*10+int(s[idx-1]-'0')-1+'a'))
			idx -= 3
		} else {
			buf = append(buf, byte('a'+s[idx]-'0'-1))
			idx--
		}
	}
	ss, e := 0, len(buf)-1
	for ss < e {
		buf[ss], buf[e] = buf[e], buf[ss]
		ss++
		e--
	}
	return string(buf)
}

func main() {
	fmt.Println("a")
}
