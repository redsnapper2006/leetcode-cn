package main

func removeOuterParentheses(S string) string {
	var stack []byte

	var ret []byte
	for i := 0; i < len(S); i++ {
		if S[i] == '(' {
			if len(stack) > 0 {
				ret = append(ret, S[i])
			}
			stack = append(stack, S[i])
		} else {
			if len(stack) > 1 {
				ret = append(ret, S[i])
			}
			stack = stack[0 : len(stack)-1]
		}
	}
	return string(ret)
}

func main() {
	fmt.Println("a")
}
