package main

import "fmt"

func makeGood(s string) string {
	buf := []byte(s)
	for {
		isValid := true
		idx := -1
		for i := 0; i < len(buf)-1; i++ {
			if buf[i]-'a' == buf[i+1]-'A' || buf[i]-'A' == buf[i+1]-'a' {
				isValid = false
				idx = i
				break
			}
		}
		if !isValid {
			buf = append(buf[0:idx], buf[idx+2:]...)
		} else {
			break
		}
	}
	return string(buf)
}

func main() {
	fmt.Println("a")
}
