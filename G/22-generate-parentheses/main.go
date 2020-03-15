package main

import (
	"fmt"
	"strings"
)

func generateParenthesis(n int) []string {
	if n == 1 {
		return []string{"()"}
	}

	var recur func(n int, stack []string, push, pop int, result *[]string)
	recur = func(n int, stack []string, push, pop int, result *[]string) {
		if push == n && pop == n {
			// fmt.Println(stack)
			*result = append(*result, strings.Join(stack, ""))
			return
		}

		if push < n {
			recur(n, append(stack, "("), push+1, pop, result)
		}

		if push > pop {
			recur(n, append(stack, ")"), push, pop+1, result)
		}
	}
	result := make([]string, 0)
	recur(n, []string{"("}, 1, 0, &result)
	return result
}

func main() {
	fmt.Println(generateParenthesis(4))
}
