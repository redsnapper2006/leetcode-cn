package main

func minRemoveToMakeValid(s string) string {
	var stack []byte
	var indexStack []int

	for i := 0; i < len(s); i++ {
		if s[i] != '(' && s[i] != ')' {
			continue
		}
		if s[i] == ')' && len(stack) > 0 && stack[len(stack)-1] == '(' {
			stack = stack[0 : len(stack)-1]
			indexStack = indexStack[0 : len(indexStack)-1]
		} else {
			stack = append(stack, s[i])
			indexStack = append(indexStack, i)
		}
	}
	if len(stack) == 0 {
		return s
	}

	r := ""
	prev := 0
	for i := 0; i < len(stack); i++ {
		r += s[prev:indexStack[i]]
		prev = indexStack[i] + 1
	}
	return r + s[prev:]
}

func main() {
	fmt.Println("a")
}
