package main

import "fmt"

func canMakePaliQueries(s string, queries [][]int) []bool {
	buf := make([][]int, len(s))
	for i := 0; i < len(s); i++ {
		buf[i] = make([]int, 26)
		if i > 0 {
			copy(buf[i], buf[i-1])
		}
		buf[i][s[i]-'a']++
	}
	ret := []bool{}
	for _, q := range queries {
		ss, e, c := q[0], q[1], q[2]
		odd := 0
		if ss == 0 {
			for i := 0; i < 26; i++ {
				if buf[e][i]%2 == 1 {
					odd++
				}
			}
		} else {
			for i := 0; i < 26; i++ {
				if (buf[e][i]-buf[ss-1][i])%2 == 1 {
					odd++
				}
			}
		}

		distance := e - ss + 1
		if distance%2 == 1 {
			odd--
		}
		r := false
		if odd/2 <= c {
			r = true
		}
		ret = append(ret, r)
	}
	return ret
}

func main() {
	fmt.Println()
}
