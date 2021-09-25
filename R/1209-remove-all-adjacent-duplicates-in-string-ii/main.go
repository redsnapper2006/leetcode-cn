package main

import "fmt"

func removeDuplicates(s string, k int) string {
	stack := []byte{}
	cntstack := []int{}
	for _, b := range s {
		if len(stack) == 0 || stack[len(stack)-1] != byte(b) || cntstack[len(cntstack)-1]+1 < k {
			stack = append(stack, byte(b))
			if len(stack) == 1 || stack[len(stack)-2] != byte(b) {
				cntstack = append(cntstack, 1)
			} else {
				cntstack = append(cntstack, cntstack[len(cntstack)-1]+1)
			}
		} else {
			stack = stack[0 : len(stack)-k+1]
			cntstack = cntstack[0 : len(cntstack)-k+1]
		}
	}
	return string(stack)
}

func main() {
	fmt.Println()
}
