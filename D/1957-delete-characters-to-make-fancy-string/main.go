package main

import "fmt"

func makeFancyString(s string) string {
	buf := []byte(s)
	base := byte('0')
	ret := []byte{}
	count := 0
	for _, v := range buf {
		if v != base {
			base = v
			count = 1
			ret = append(ret, base)
		} else {
			count++
			if count <= 2 {
				ret = append(ret, base)
			}
		}
	}
	return string(ret)
}

func main() {
	fmt.Println()
}
