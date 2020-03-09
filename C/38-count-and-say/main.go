package main

import "fmt"

import "strconv"

func countAndSay(n int) string {
	b := []string{"1", "1"}
	for i := 2; i <= n; i++ {
		s := b[i-1]
		t := s[0]
		c := 1
		r := ""
		for j := 1; j < len(s); j++ {
			if t == s[j] {
				c++
			} else {
				r += strconv.FormatInt(int64(c), 10) + string(t)
				t = s[j]
				c = 1
			}
		}
		r += strconv.FormatInt(int64(c), 10) + string(t)
		b = append(b, r)
	}
	return b[n]
}

func main() {
	fmt.Println(countAndSay(5))
}
