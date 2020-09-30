package main

import "fmt"

func maxProduct(words []string) int {
	buf := make([]int, len(words))
	for i, v := range words {
		s := 0
		for _, c := range v {
			s |= 1 << int(c-'a')
		}
		buf[i] = s
	}
	max := 0
	for i := 0; i < len(buf); i++ {
		for j := i + 1; j < len(buf); j++ {
			if buf[i]&buf[j] == 0 && max < len(words[i])*len(words[j]) {
				max = len(words[i]) * len(words[j])
			}
		}
	}
	return max
}

func maxProductV2(words []string) int {
	buf := make([][]int, len(words))
	for i, v := range words {
		buf[i] = make([]int, 26)
		for _, c := range v {
			buf[i][c-'a']++
		}
	}
	max := 0
	for i := 0; i < len(buf); i++ {
		for j := i + 1; j < len(buf); j++ {
			isValid := true
			for n := 0; n < 26; n++ {
				if buf[i][n] > 0 && buf[j][n] > 0 {
					isValid = false
					break
				}
			}
			if isValid && max < len(words[i])*len(words[j]) {
				max = len(words[i]) * len(words[j])
			}
		}
	}
	return max
}

func main() {
	fmt.Println("a")
}
