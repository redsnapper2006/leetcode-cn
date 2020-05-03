package main

import "fmt"

func validateStackSequences(pushed []int, popped []int) bool {
	pushIdx, popIdx := 0, 0

	var stack []int
	for {
		if len(stack) == 0 {
			if pushIdx >= len(pushed) {
				return true
			}
			stack = append(stack, pushed[pushIdx])
			pushIdx++
		} else {
			if stack[len(stack)-1] == popped[popIdx] {
				stack = stack[0 : len(stack)-1]
				popIdx++
			} else {
				if pushIdx >= len(pushed) {
					return false
				}
				stack = append(stack, pushed[pushIdx])
				pushIdx++
			}
		}
	}
}

func main() {
	fmt.Println("a")
}
