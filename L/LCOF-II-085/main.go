package main

import (
	"fmt"
)

func generateParenthesis(n int) []string {
	ret := []string{}
	var recur func(push, pop int, s string)
	recur = func(push, pop int, s string) {
		if push == n && pop == push {
			ret = append(ret, s)
			return
		}
		if push < n {
			recur(push+1, pop, s+"(")
		}
		if pop < push {
			recur(push, pop+1, s+")")
		}
	}
	recur(0, 0, "")
	return ret
}

func main() {
	fmt.Println()
}
