package main

import (
	"fmt"
)

func isIsomorphic(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}

	buf1 := make(map[byte]byte)
	buf2 := make(map[byte]byte)
	for i := 0; i < len(s); i++ {
		v1, ok1 := buf1[s[i]]
		_, ok2 := buf2[t[i]]

		if ok1 != ok2 {
			return false
		} else {
			if !ok1 && !ok2 {
				buf1[s[i]] = t[i]
				buf2[t[i]] = s[i]
			} else {
				if v1 != t[i] {
					return false
				}
			}
		}
	}
	return true
}

func main() {
	fmt.Println(isIsomorphic("egg", "add"))
	fmt.Println(isIsomorphic("foo", "bar"))

}
