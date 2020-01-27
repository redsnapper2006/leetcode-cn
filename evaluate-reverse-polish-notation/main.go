package main

import (
	"fmt"
	"strconv"
)

func evalRPN(tokens []string) int {

	for i := 0; i < len(tokens); i++ {
		if tokens[i] == "+" || tokens[i] == "-" || tokens[i] == "*" || tokens[i] == "/" {
			o2, _ := strconv.Atoi(tokens[i-1])
			o1, _ := strconv.Atoi(tokens[i-2])
			var r int
			if tokens[i] == "+" {
				r = o1 + o2
			}
			if tokens[i] == "-" {
				r = o1 - o2
			}
			if tokens[i] == "*" {
				r = o1 * o2
			}
			if tokens[i] == "/" {
				r = o1 / o2
			}
			tokens = append(tokens[0:i-1], tokens[i+1:]...)
			tokens[i-2] = strconv.FormatInt(int64(r), 10)
			i -= 2
			// fmt.Println(i, tokens)
		}
	}
	r, _ := strconv.Atoi(tokens[0])
	return r
}

func main() {
	fmt.Println(evalRPN([]string{"10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"}))
	fmt.Println(evalRPN([]string{"1", "2", "3", "4", "5", "6", "+", "+", "+", "+", "+"}))
}
