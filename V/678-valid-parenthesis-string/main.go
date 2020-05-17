package main

import "fmt"

func checkValidString(s string) bool {
	lo, hi := 0, 0
	for i := 0; i < len(s); i++ {
		if s[i] == '(' {
			lo++
			hi++
		} else if s[i] == ')' {
			if lo > 0 {
				lo--
			}
			if hi == 0 {
				return false
			} else {
				hi--
			}
		} else {
			hi++
			if lo > 0 {
				lo--
			}
		}
	}

	return lo == 0
}

func checkValidStringV2(s string) bool {
	var recur func(count int, previous byte, s string) bool
	recur = func(count int, previous byte, s string) bool {
		if len(s) == 0 {
			return count == 0
		}
		if s[0] == '(' {
			return recur(count+1, '(', s[1:])
		} else if s[0] == ')' {
			if count < 1 {
				return false
			}
			// fmt.Println("right", stack, s)
			if previous == '(' {
				return recur(count-1, ')', s[1:])
			} else {
				return recur(count-1, ')', s[1:])
			}
		} else {
			empty := recur(count, previous, s[1:])
			left := recur(count+1, '(', s[1:])
			right := recur(count-1, ')', s[1:])
			return empty || left || right
		}
	}
	return recur(0, ' ', s)
}

func main() {
	// fmt.Println(checkValidString("(*))"))
	// fmt.Println(checkValidString("(*)"))

	fmt.Println(checkValidString("(())((())()()(*)(*()(())())())()()((()())((()))(*"))
	fmt.Println(checkValidString("((*)(*))((*"))
	fmt.Println(checkValidString("((*)(*))((*"))
}
