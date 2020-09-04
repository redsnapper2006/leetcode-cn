package main

import (
	"fmt"
	"sort"
	"strings"
)

type StrSlice []string

func (p StrSlice) Len() int {
	return len(p)
}

func (p StrSlice) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p StrSlice) Less(i, j int) bool {
	a, b := p[i], p[j]
	arrA := strings.Split(a, " ")
	arrB := strings.Split(b, " ")
	kA := strings.Join(arrA[1:], " ")
	kB := strings.Join(arrB[1:], " ")
	if kA != kB {
		return kA < kB
	}
	return arrA[0] < arrB[0]
}

func reorderLogFiles(logs []string) []string {
	var digit []string
	var literal []string
	for _, v := range logs {
		arr := strings.Split(v, " ")
		isAllDigit := true
		for j := 1; j < len(arr); j++ {
			if arr[j] == "" {
				continue
			}
			isDigit := true
			for n := 0; n < len(arr[j]); n++ {
				if !(arr[j][n] >= '0' && arr[j][n] <= '9') {
					isDigit = false
					break
				}
			}
			if !isDigit {
				isAllDigit = false
				break
			}
		}
		if isAllDigit {
			digit = append(digit, v)
		} else {
			literal = append(literal, v)
		}
	}
	sort.Sort(StrSlice(literal))
	var buf []string
	for _, v := range literal {
		buf = append(buf, v)
	}
	for _, v := range digit {
		buf = append(buf, v)
	}
	return buf
}

func main() {
	fmt.Println("a")
}
