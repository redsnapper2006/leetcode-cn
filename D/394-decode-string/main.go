package main

import "fmt"

func decodeString(s string) string {
	numStack := []int{}
	byteStack := []byte{}
	bytePos := []int{}
	count := 0
	for i := 0; i < len(s); i++ {
		if s[i] >= '0' && s[i] <= '9' {
			count = count*10 + int(s[i]-'0')
		} else {
			if s[i] == '[' {
				numStack = append(numStack, count)
				count = 0
				//byteStack = append(byteStack, s[i])
				bytePos = append(bytePos, len(byteStack))
			} else if s[i] == ']' {
				prevPos := bytePos[len(bytePos)-1]
				endPos := len(byteStack)
				count := numStack[len(numStack)-1]
				for j := 1; j < count; j++ {
					for m := prevPos; m < endPos; m++ {
						byteStack = append(byteStack, byteStack[m])
					}
				}
				bytePos = bytePos[0 : len(bytePos)-1]
				numStack = numStack[0 : len(numStack)-1]
			} else {
				byteStack = append(byteStack, s[i])
			}
		}
	}

	return string(byteStack)
}

func main() {
	fmt.Println(decodeString("3[a]2[bc]"))
	fmt.Println(decodeString("3[a2[c]]"))
	fmt.Println(decodeString("2[abc]3[cd]ef"))
}
