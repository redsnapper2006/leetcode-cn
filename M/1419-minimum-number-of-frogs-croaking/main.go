package main

import "fmt"

func minNumberOfFrogs(croakOfFrogs string) int {
	offset := 0
	cnt := -1
	stack := make([][]byte, 5)
	for _, b := range croakOfFrogs {
		idx := -1
		if b == 'c' {
			idx = 0
		} else if b == 'r' {
			idx = 1
		} else if b == 'o' {
			idx = 2
		} else if b == 'a' {
			idx = 3
		} else {
			idx = 4
		}
		if idx > 0 {
			if len(stack[idx]) >= len(stack[idx-1]) {
				return -1
			}
		}
		stack[idx] = append(stack[idx], byte(b))
		if idx == 4 {
			if len(stack[0])-offset > cnt {
				cnt = len(stack[0]) - offset
			}
			offset++
		}
	}
	for _, b := range stack {
		if len(b) > offset {
			return -1
		}
	}
	return cnt
}

func main() {
	fmt.Println()
}
