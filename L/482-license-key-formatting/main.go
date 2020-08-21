package main

import "fmt"

func licenseKeyFormatting(S string, K int) string {
	var buf []byte
	for i := 0; i < len(S); i++ {
		if S[i] == '-' {
			continue
		}
		if S[i] <= 'z' && S[i] >= 'a' {
			buf = append(buf, S[i]-'a'+'A')
		} else {
			buf = append(buf, S[i])
		}
	}
	var ret []byte
	for i := len(buf) - 1; i >= 0; i = i - K {
		for j := 0; j < K && i-j >= 0; j++ {
			ret = append(ret, buf[i-j])
		}
		ret = append(ret, '-')
	}
	if len(ret) == 0 {
		return ""
	}
	ret = ret[0 : len(ret)-1]
	s, e := 0, len(ret)-1
	for s < e {
		ret[s], ret[e] = ret[e], ret[s]
		s++
		e--
	}
	return string(ret)
}

func main() {
	fmt.Println("a")
}
