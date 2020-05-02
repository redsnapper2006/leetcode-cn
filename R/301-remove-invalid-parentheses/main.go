package main

import (
	"fmt"
)

func removeInvalidParentheses(s string) []string {
	var stack []byte
	var indexStack []int

	for i := 0; i < len(s); i++ {
		if s[i] == '(' {
			stack = append(stack, s[i])
			indexStack = append(indexStack, i)
		} else if s[i] == ')' {
			if len(stack) > 0 && stack[len(stack)-1] == '(' {
				stack = stack[0 : len(stack)-1]
				indexStack = indexStack[0 : len(indexStack)-1]
			} else {
				stack = append(stack, s[i])
				indexStack = append(indexStack, i)
			}
		}
	}

	if len(stack) == 0 {
		return []string{s}
	}

	leftBracketIndex := -1
	for i := 0; i < len(stack); i++ {
		if stack[i] == '(' {
			leftBracketIndex = i
			break
		}
	}
	rightSubStr, leftSubStr, midSubStr := "", "", ""
	rightCnt, leftCnt := 0, 0
	if leftBracketIndex == -1 {
		rightSubStr = s[0 : indexStack[len(indexStack)-1]+1]
		rightCnt = len(indexStack)
		midSubStr = s[indexStack[len(indexStack)-1]+1:]
	} else {
		if leftBracketIndex == 0 {
			midSubStr = s[0:indexStack[0]]
			leftSubStr = s[indexStack[0]:]
			leftCnt = len(indexStack)
		} else {
			rightSubStr = s[0 : indexStack[leftBracketIndex-1]+1]
			midSubStr = s[indexStack[leftBracketIndex-1]+1 : indexStack[leftBracketIndex]]
			leftSubStr = s[indexStack[leftBracketIndex]:]
			rightCnt = leftBracketIndex
			leftCnt = len(indexStack) - leftBracketIndex
		}
	}

	var recur func(str []byte, target byte, remainedCnt, removeCnt int) [][]byte
	recur = func(str []byte, target byte, remainedCnt, removeCnt int) [][]byte {
		if removeCnt == 0 {
			return [][]byte{str}
		}
		if remainedCnt == removeCnt {
			var buf []byte
			for i := 0; i < len(str); i++ {
				if str[i] != target {
					buf = append(buf, str[i])
				}
			}
			return [][]byte{buf}
		}
		idx := -1
		for i := 0; i < len(str); i++ {
			if str[i] == target {
				idx = i
				break
			}
		}
		var ret [][]byte
		pRemove := str[0:idx]
		pRemained := recur(str[idx+1:], target, remainedCnt-1, removeCnt-1)
		for i := 0; i < len(pRemained); i++ {
			var b []byte
			for j := 0; j < len(pRemove); j++ {
				b = append(b, pRemove[j])
			}
			for j := 0; j < len(pRemained[i]); j++ {
				b = append(b, pRemained[i][j])
			}
			ret = append(ret, b)
		}
		pKeep := str[0 : idx+1]
		pKeepRemained := recur(str[idx+1:], target, remainedCnt-1, removeCnt)
		for i := 0; i < len(pKeepRemained); i++ {
			var b []byte
			for j := 0; j < len(pKeep); j++ {
				b = append(b, pKeep[j])
			}
			for j := 0; j < len(pKeepRemained[i]); j++ {
				b = append(b, pKeepRemained[i][j])
			}
			ret = append(ret, b)
		}
		return ret
	}

	rightRemainedCnt := 0
	for i := 0; i < len(rightSubStr); i++ {
		if rightSubStr[i] == ')' {
			rightRemainedCnt++
		}
	}
	leftRemainedCnt := 0
	for i := 0; i < len(leftSubStr); i++ {
		if leftSubStr[i] == '(' {
			leftRemainedCnt++
		}
	}

	rightByteResult := recur([]byte(rightSubStr), ')', rightRemainedCnt, rightCnt)
	leftByteResult := recur([]byte(leftSubStr), '(', leftRemainedCnt, leftCnt)

	rightResult := make(map[string]int)
	for i := 0; i < len(rightByteResult); i++ {
		c := 0
		for j := 0; j < len(rightByteResult[i]); j++ {
			if rightByteResult[i][j] == '(' {
				c++
			}
			if rightByteResult[i][j] == ')' {
				c--
			}
			if c < 0 {
				break
			}
		}
		if c >= 0 {
			rightResult[string(rightByteResult[i])]++
		}
	}
	leftResult := make(map[string]int)
	for i := 0; i < len(leftByteResult); i++ {
		c := 0
		for j := 0; j < len(leftByteResult[i]); j++ {
			if leftByteResult[i][j] == '(' {
				c++
			}
			if leftByteResult[i][j] == ')' {
				c--
			}
			if c < 0 {
				break
			}
		}
		if c >= 0 {
			leftResult[string(leftByteResult[i])]++
		}
	}

	var ret []string
	for k, _ := range rightResult {
		for k2, _ := range leftResult {
			ret = append(ret, k+midSubStr+k2)
		}
	}

	return ret
}

func main() {
	fmt.Println(removeInvalidParentheses("()())()"))
	fmt.Println(removeInvalidParentheses("(a)())()"))
	fmt.Println(removeInvalidParentheses(")("))
	fmt.Println(removeInvalidParentheses(")(f"))
	fmt.Println(removeInvalidParentheses(")()(f"))
	fmt.Println(removeInvalidParentheses("(()("))
	fmt.Println(removeInvalidParentheses("(((k()(("))
	fmt.Println(removeInvalidParentheses("(r(()()("))
}
