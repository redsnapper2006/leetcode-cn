package main

import (
	"fmt"
	"strings"
)

func uncommonFromSentences(A string, B string) []string {
	MA := map[string]int{}
	arrA := strings.Split(A, " ")
	for _, v := range arrA {
		MA[v]++
	}
	MB := map[string]int{}
	arrB := strings.Split(B, " ")
	for _, v := range arrB {
		MB[v]++
	}
	for k, v := range MA {
		if v > 1 {
			delete(MA, k)
			delete(MB, k)
		}
	}
	for k, v := range MB {
		if v > 1 {
			delete(MA, k)
			delete(MB, k)
		}
	}
	var buf []string
	for k := range MA {
		_, ok := MB[k]
		if !ok {
			buf = append(buf, k)
		}
	}
	for k := range MB {
		_, ok := MA[k]
		if !ok {
			buf = append(buf, k)
		}
	}
	return buf
}

func main() {
	fmt.Println("a")
}
