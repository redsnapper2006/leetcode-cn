package main

import (
	"fmt"
	"strings"
)

func discountPrices(sentence string, discount int) string {
	arr := strings.Split(sentence, " ")
	for idx, v := range arr {
		if v[0] != byte('$') || len(v) == 1 {
			continue
		}
		n := 0
		isDigit := true
		for i := 1; i < len(v); i++ {
			if v[i] < byte('0') || v[i] > byte('9') {
				isDigit = false
				break
			}
			n = n*10 + int(v[i]-'0')
		}
		if !isDigit {
			continue
		}
		arr[idx] = fmt.Sprintf("$%.2f", float64(n)*float64(100-discount)/float64(100))
	}
	return strings.Join(arr, " ")
}
