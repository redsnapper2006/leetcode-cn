package main

import (
	"fmt"
)

func shortestCompletingWord(licensePlate string, words []string) string {

	buf := make([]int, 26)
	for _, v := range licensePlate {
		if v >= 'a' && v <= 'z' {
			buf[v-'a']++
		}
		if v >= 'A' && v <= 'Z' {
			buf[v-'A']++
		}
	}

	s := "1234567890123456"
	for _, v := range words {
		vb := make([]int, 26)
		for _, vl := range v {
			vb[vl-'a']++
		}

		isMatch := true
		for i := 0; i < 26; i++ {
			if buf[i] > 0 && buf[i] > vb[i] {
				isMatch = false
				break
			}
		}

		if isMatch && len(s) > len(v) {
			s = v
		}

	}
	return s
}

func main() {
	fmt.Println(shortestCompletingWord("1s3 PSt", []string{"step", "steps", "stripe", "stepple"}))
	fmt.Println(shortestCompletingWord("1s3 456", []string{"looks", "pest", "stew", "show"}))

}
