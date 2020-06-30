package main

import "fmt"

func compress(chars []byte) int {
	bIdx := 0
	c := 0
	b := chars[0]
	for i := 0; i < len(chars); i++ {
		if chars[i] != b {
			if c == 1 {
				chars[bIdx] = b
				bIdx++
			} else {
				chars[bIdx] = b
				var t []int
				for c > 0 {
					m := c % 10
					t = append(t, m)
					c /= 10
				}
				for j := 0; j < len(t); j++ {
					chars[bIdx+1+j] = byte(t[len(t)-1-j] + int('0'))
				}
				bIdx += len(t) + 1
			}
			b = chars[i]
			c = 0
		}
		c++
	}
	if c == 1 {
		chars[bIdx] = b
		bIdx++
	} else {
		chars[bIdx] = b
		var t []int
		for c > 0 {
			m := c % 10
			t = append(t, m)
			c /= 10
		}
		for j := 0; j < len(t); j++ {
			chars[bIdx+1+j] = byte(t[len(t)-1-j] + int('0'))
		}
		bIdx += len(t) + 1
	}
	return bIdx
}

func main() {
	fmt.Println("a")
}
