package main

import (
	"fmt"
)

func frequencySort(s string) string {
	buf := make(map[byte]int)
	for i := 0; i < len(s); i++ {
		buf[s[i]]++
	}
	max := 0
	for _, v := range buf {
		if v > max {
			max = v
		}
	}

	buf2 := make([][]byte, max+1)

	for k, v := range buf {
		buf2[v] = append(buf2[v], k)
	}

	var r []byte
	for i := len(buf2) - 1; i > 0; i-- {
		for _, l := range buf2[i] {
			for j := 0; j < i; j++ {
				r = append(r, l)
			}
		}
	}
	return string(r)
}

func main() {
	fmt.Println(frequencySort("tree"))
}
