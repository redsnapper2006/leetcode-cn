package main

import "fmt"

func maxDepthAfterSplit(seq string) []int {
	var ans []int
	var stack []byte
	for i := 0; i < len(seq); i++ {
		if seq[i] == '(' {
			stack = append(stack, seq[i])
			ans = append(ans, len(stack)%2)
		}
		if seq[i] == ')' {
			ans = append(ans, len(stack)%2)
			stack = stack[0 : len(stack)-1]
		}
	}

	return ans
}

func maxDepthAfterSplitV2(seq string) []int {
	var depthStack []int
	var stack []byte
	for i := 0; i < len(seq); i++ {
		if seq[i] == '(' {
			stack = append(stack, seq[i])
			depthStack = append(depthStack, len(stack))
		}
		if seq[i] == ')' {
			depthStack = append(depthStack, len(stack))
			stack = stack[0 : len(stack)-1]
		}
	}
	ans := make([]int, len(depthStack))
	for i := 0; i < len(depthStack); i++ {
		if depthStack[i]%2 == 0 {
			ans[i] = 0
		} else {
			ans[i] = 1
		}
	}

	return ans
}

func main() {
	fmt.Println(maxDepthAfterSplit("(((()))((())))"))
}
