package main

import (
	"fmt"
)

func convert(s string, numRows int) string {
	if numRows == 1 {
		return s
	}
	fMode := 2*numRows - 2
	ret := make([]string, numRows)

	for i := 0; i < numRows; i++ {
		for j := 0; j < len(s); j++ {
			if j%fMode == i || j%fMode == fMode-i {
				ret[i] += string(s[j])
			}
		}
	}
	r := ""
	for i := 0; i < numRows; i++ {
		r += ret[i]
	}
	return r
}

func main() {
	fmt.Println(convert("A",1))
}
