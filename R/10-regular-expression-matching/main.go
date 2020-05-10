package main

import "fmt"

func isMatch(s string, p string) bool {
	buf := make([][]int, len(p)+1)
	for i := 0; i < len(p)+1; i++ {
		buf[i] = make([]int, len(s)+1)
	}
	buf[0][0] = 1
	for i := 1; i < len(s)+1; i++ {
		buf[0][i] = 0
	}
	for i := 1; i < len(p)+1; i++ {
		if p[i-1] == '*' && (buf[i-1][0] == 1 || buf[i-2][0] == 1) {
			buf[i][0] = 1
		} else {
			buf[i][0] = 0
		}
	}

	for i := 1; i < len(p)+1; i++ {
		for j := 1; j < len(s)+1; j++ {
			if p[i-1] == '.' {
				buf[i][j] = buf[i-1][j-1]
			} else if p[i-1] == '*' && (buf[i-1][j] == 1 || buf[i-2][j] == 1 || buf[i-1][j-1] == 1 && (p[i-2] == '.' || s[j-2] == s[j-1]) ||
				(buf[i][j-1] == 1 && p[i-2] == '.')) {
				buf[i][j] = 1
			} else if p[i-1] == s[j-1] {
				buf[i][j] = buf[i-1][j-1]
			}
		}
	}
	return buf[len(p)][len(s)] == 1
}

func main() {
	fmt.Println(isMatch("aab", "c*a*b"))
}
