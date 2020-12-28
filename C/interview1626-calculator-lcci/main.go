package main

import "fmt"

func calculate(s string) int {

	var buf []byte
	for _, b := range s {
		if byte(b) == ' ' {
			continue
		}
		buf = append(buf, byte(b))
	}

	var stack []int
	var ops []byte
	num := 0
	for _, b := range buf {
		// fmt.Println("loop", string(b))
		if byte(b) == '+' || byte(b) == '-' || byte(b) == '*' || byte(b) == '/' {
			// fmt.Println("+", stack, string(ops))
			if len(ops) > 0 && (ops[len(ops)-1] == '*' || ops[len(ops)-1] == '/') {
				t := 0
				if ops[len(ops)-1] == '*' {
					t = stack[len(stack)-1] * num
				}
				if ops[len(ops)-1] == '/' {
					t = stack[len(stack)-1] / num
				}
				ops = ops[0 : len(ops)-1]
				stack[len(stack)-1] = t
			} else {
				stack = append(stack, num)
			}
			ops = append(ops, byte(b))
			num = 0
			continue
		}
		num = num*10 + int(b-'0')
	}
	// fmt.Println("out", stack, string(ops))
	if len(ops) > 0 && (ops[len(ops)-1] == '*' || ops[len(ops)-1] == '/') {
		t := 0
		if ops[len(ops)-1] == '*' {
			t = stack[len(stack)-1] * num
		}
		if ops[len(ops)-1] == '/' {
			t = stack[len(stack)-1] / num
		}
		ops = ops[0 : len(ops)-1]
		stack[len(stack)-1] = t
	} else {
		stack = append(stack, num)
	}
	// fmt.Println("first", stack, string(ops))
	first := stack[0]
	for i := 0; i < len(ops); i++ {
		if ops[i] == '+' {
			first += stack[i+1]
		}
		if ops[i] == '-' {
			first -= stack[i+1]
		}
	}
	return first
}

func main() {
	fmt.Println()
}
