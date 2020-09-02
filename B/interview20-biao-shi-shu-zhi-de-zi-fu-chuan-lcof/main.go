package main

import (
	"fmt"
	"strings"
)

func isNumber(s string) bool {
	s = strings.TrimSpace(s)
	if len(s) == 0 {
		return false
	}
	s = strings.ToLower(s)
	arr := strings.Split(s, "e")
	if len(arr) > 2 {
		return false
	}
	buf := make([]byte, len(arr[0]))
	copy(buf, arr[0])
	if len(buf) == 0 {
		return false
	}
	sIdx := 0
	if buf[0] == '-' || buf[0] == '+' {
		sIdx = 1
	}
	if sIdx >= len(buf) {
		return false
	}
	if sIdx+1 >= len(buf) && buf[sIdx] == '.' {
		return false
	}
	if !((buf[sIdx] <= '9' && buf[sIdx] >= '0') || (buf[sIdx] == '.')) {
		return false
	}

	isFirstDot := true
	for i := sIdx; i < len(buf); i++ {
		if buf[i] == '-' || buf[i] == '+' {
			return false
		}
		if buf[i] == '.' {
			if isFirstDot {
				isFirstDot = false
				continue
			} else {
				return false
			}
		}
		if !(buf[i] <= '9' && buf[i] >= '0') {
			return false
		}
	}

	if len(arr) == 2 {
		buf := make([]byte, len(arr[1]))
		copy(buf, arr[1])
		if len(buf) == 0 {
			return false
		}

		sIdx = 0
		if buf[0] == '-' || buf[0] == '+' {
			sIdx = 1
		}
		if sIdx >= len(buf) {
			return false
		}
		if sIdx+1 >= len(buf) && buf[sIdx] == '.' {
			return false
		}
		if !((buf[sIdx] <= '9' && buf[sIdx] >= '0') || (buf[sIdx] == '.')) {
			return false
		}

		for i := sIdx; i < len(buf); i++ {
			if buf[i] == '-' || buf[i] == '+' {
				return false
			}
			if buf[i] == '.' {
				return false
			}
			if buf[i] == 'e' {
				return false
			}
			if !(buf[i] <= '9' && buf[i] >= '0') {
				return false
			}
		}
	}

	return true
}

func main() {
	fmt.Println(isNumber(".1"))
}
