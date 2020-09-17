package main

import "fmt"

func modifyString(s string) string {

	var buf []byte
	occu := make([]int, 26)
	c := 0
	for i, b := range s {
		if b == '?' {
			if c == 0 {
				occu = make([]int, 26)
				if i > 0 {
					occu[s[i-1]-'a'] = 1
				}
			}
			c++
		} else {
			if c > 0 {
				occu[b-'a'] = 1
				cx, cy := byte('0'), byte('0')
				for m := 0; m < len(occu); m++ {
					if occu[m] == 1 {
						continue
					}
					if cx == '0' {
						cx = byte('a' + m)
						continue
					}
					if cy == '0' {
						cy = byte('a' + m)
						break
					}
				}
				for n := 0; n < c; n++ {
					t := cx
					if n%2 == 1 {
						t = cy
					}
					buf = append(buf, t)
				}
			}
			buf = append(buf, byte(b))
			c = 0
		}
	}
	if c > 0 {
		cx, cy := byte('0'), byte('0')
		for m := 0; m < len(occu); m++ {
			if occu[m] == 1 {
				continue
			}
			if cx == '0' {
				cx = byte('a' + m)
				continue
			}
			if cy == '0' {
				cy = byte('a' + m)
				break
			}
		}
		for n := 0; n < c; n++ {
			t := cx
			if n%2 == 1 {
				t = cy
			}
			buf = append(buf, t)
		}
	}
	return string(buf)
}

func main() {
	fmt.Println()
}
