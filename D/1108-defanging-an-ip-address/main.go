package main

import "fmt"

func defangIPaddr(address string) string {
	var buf []byte
	for _, v := range address {
		if v == '.' {
			buf = append(buf, '[', byte(v), ']')
		} else {
			buf = append(buf, byte(v))
		}
	}
	return string(buf)
}

func main() {
	fmt.Println("a")
}
