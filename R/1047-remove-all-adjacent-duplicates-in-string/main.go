package main

import "fmt"

func removeDuplicates(S string) string {
	var stack []byte
	for _, v := range S {
		if len(stack) > 0 && stack[len(stack)-1] == byte(v) {
			stack = stack[0 : len(stack)-1]
			continue
		} else {
			stack = append(stack, byte(v))
		}
	}
	return string(stack)
}

func removeDuplicatesV2(S string) string {
	buf := make([]byte, len(S))
	copy(buf, S)
	if len(buf) == 0 {
		return ""
	}
	for {
		idx := -1
		for i := 1; i < len(buf); i++ {
			if buf[i] == buf[i-1] {
				idx = i
				break
			}
		}
		if idx != -1 {
			buf = append(buf[0:idx-1], buf[idx+1:]...)
		} else {
			return string(buf)
		}

	}
}

func main() {
	fmt.Println("a")
}
