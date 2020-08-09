package main

import "fmt"

func sortString(s string) string {
	buf := make([]int, 26)
	for i := 0; i < len(s); i++ {
		buf[s[i]-'a']++
	}
	round := 0
	var b []byte
	for {
		isFound := false
		if round%2 == 0 {
			for i := 0; i < 26; i++ {
				if buf[i] > 0 {
					b = append(b, byte(i+'a'))
					buf[i]--
					isFound = true
				}
			}
		} else {
			for i := 25; i >= 0; i-- {
				if buf[i] > 0 {
					b = append(b, byte(i+'a'))
					buf[i]--
					isFound = true
				}
			}
		}
		if !isFound {
			break
		}
		round++
	}
	return string(b)
}

func main() {
	fmt.Println("a")
}
