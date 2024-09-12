package main

import "fmt"

func s2b(ss string) string {
	bb := []byte(ss)
	sum := 0
	for _, b := range bb {
		d := int(b - '0')
		sum = sum*10 + d
	}
	ans := []byte{}
	for sum > 0 {
		ans = append(ans, byte(sum%2+'0'))
		sum /= 2
	}
	s, e := 0, len(ans)-1
	for s < e {
		t := ans[s]
		ans[s] = ans[e]
		ans[e] = t
		s += 1
		e -= 1
	}
	return string(ans)
}

func convertDateToBinary(date string) string {
	return s2b(date[0:4]) + "-" + s2b(date[5:7]) + "-" + s2b(date[8:])
}

func main() {
	fmt.Println(convertDateToBinary("1901-01-11"))
}
