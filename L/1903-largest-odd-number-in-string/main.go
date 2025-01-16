package main

import "fmt"

func largestOddNumber(num string) string {
	buf := []byte(num)
	idx := -1
	for i := len(num) - 1; i >= 0; i-- {
		if int(buf[i]-'0')%2 == 0 {
			continue
		}
		idx = i
		break
	}
	if idx == -1 {
		return ""
	}
	return string(buf[0 : idx+1])
}

func main() {
	fmt.Println()
}
