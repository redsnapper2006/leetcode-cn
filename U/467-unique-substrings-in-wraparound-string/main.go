package main

import "fmt"

func findSubstringInWraproundString(p string) int {
	if len(p) == 0 {
		return 0
	}
	r := 0
	c := 0
	buf := make([]int, 26)
	buf[p[0]-'a']++
	M := map[string]int{}
	for i := 1; i < len(p); i++ {
		if p[i]-p[i-1] == 1 || (p[i] == 'a' && p[i-1] == 'z') {
			c++
		} else {
			if c > 0 {
				for m := 0; m < 26 && m < c; m++ {
					for j := i - c + m + 1; j <= i; j++ {
						_, ok := M[p[i-c-1+m:j]]
						if !ok {
							r++
							M[p[i-c-1+m:j]]++
						}
					}
				}
			}
			c = 0
		}
		buf[p[i]-'a']++
	}

	if c > 0 {
		for m := 0; m < 26 && m < c; m++ {
			for j := len(p) - c + m + 1; j <= len(p); j++ {
				_, ok := M[p[len(p)-c-1+m:j]]
				if !ok {
					r++
					M[p[len(p)-c-1+m:j]]++
				}
			}
		}
	}
	for i := 0; i < 26; i++ {
		if buf[i] > 0 {
			r++
		}
	}
	return r
}

func main() {
	fmt.Println("a")
}
