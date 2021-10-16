package main

import "fmt"

func addOperators(num string, target int) []string {
	ret := []string{}

	fullStack := []byte{}
	var recur func(num string, fullStack []byte, isSymbol bool)
	recur = func(num string, fullStack []byte, isSymbol bool) {
		if len(num) == 0 {
			n := 0
			stack := []int{}
			oper := []byte{}
			for i := 0; i < len(fullStack); i++ {
				if fullStack[i] >= byte('0') && fullStack[i] <= byte('9') {
					if fullStack[i] == byte('0') && n == 0 && i < len(fullStack)-1 && fullStack[i+1] >= byte('0') && fullStack[i] <= byte('9') {
						return
					}
					n = n*10 + int(fullStack[i]-byte('0'))
				} else {
					if len(stack) == 0 {
						stack = append(stack, n)
						oper = append(oper, fullStack[i])
					} else if fullStack[i] == byte('+') || fullStack[i] == byte('-') {
						prev := n
						idx := len(stack) - 1
						for idx >= 0 {
							if oper[idx] == byte('+') {
								prev = stack[idx] + prev
							} else if oper[idx] == byte('-') {
								prev = stack[idx] - prev
							} else if oper[idx] == byte('*') {
								prev = stack[idx] * prev
							}
							idx--
						}
						stack = []int{prev}
						oper = []byte{fullStack[i]}
					} else if fullStack[i] == byte('*') {
						prev := n
						idx := len(stack) - 1
						for idx >= 0 && oper[idx] == byte('*') {

							prev = stack[idx] * prev
							idx--
						}
						stack = stack[0 : idx+1]
						stack = append(stack, prev)
						oper = oper[0 : idx+1]
						oper = append(oper, fullStack[i])
					}
					n = 0
				}
			}
			prev := n
			idx := len(stack) - 1
			for idx >= 0 {
				if oper[idx] == byte('+') {
					prev = stack[idx] + prev
				} else if oper[idx] == byte('-') {
					prev = stack[idx] - prev
				} else if oper[idx] == byte('*') {
					prev = stack[idx] * prev
				}
				idx--
			}
			// fmt.Println(string(fullStack), prev, target)
			if prev == target {
				ret = append(ret, string(fullStack))
			}
			return
		}
		// calc
		fullStack = append(fullStack, num[0])
		recur(num[1:], fullStack, false)

		if !isSymbol {
			// +
			fullStack[len(fullStack)-1] = byte('+')
			recur(num, fullStack, true)
			//-
			fullStack[len(fullStack)-1] = byte('-')
			recur(num, fullStack, true)
			// *
			fullStack[len(fullStack)-1] = byte('*')
			recur(num, fullStack, true)
			fullStack = fullStack[0 : len(fullStack)-1]
		}
	}
	recur(num, fullStack, true)
	return ret
}

func main() {
	fmt.Println(addOperators("123", 6))
	fmt.Println(addOperators("232", 8))
	fmt.Println(addOperators("105", 5))
	fmt.Println(addOperators("00", 0))
	fmt.Println(addOperators("3456237490", 9191))
}
