package main

import "fmt"

func myAtoi(str string) int {
	sIdx := -1

	for i := 0; i < len(str); i++ {
		if str[i] != ' ' {
			sIdx = i
			break
		}
	}
	if sIdx == -1 || (sIdx == len(str)-1 && (str[sIdx] == '-' || str[sIdx] == '+')) || !(str[sIdx] == '-' || str[sIdx] == '+' || (str[sIdx] <= '9' && str[sIdx] >= '0')) {
		return 0
	}

	isMinus := false
	if str[sIdx] == '-' || str[sIdx] == '+' {
		if str[sIdx] == '-' {
			isMinus = true
		}
		sIdx++
	}
	if !(str[sIdx] <= '9' && str[sIdx] >= '0') {
		return 0
	}

	n := 0
	for i := sIdx; i < len(str); i++ {
		if str[i] <= '9' && str[i] >= '0' {
			if !isMinus {
				if (1<<31 - 1 - n*10) < int(str[i]-'0') {
					return 1<<31 - 1
				}
			} else {
				if (1<<31 - n*10) < int(str[i]-'0') {
					return -1 << 31
				}
			}
			n = n*10 + int(str[i]-'0')
		} else {
			break
		}
	}
	if isMinus {
		n = -n
	}
	return n
}

func main() {
	fmt.Println(myAtoi("-91283472332"))
	fmt.Println(myAtoi("words and 987"))
	fmt.Println(myAtoi("4193 with words"))
	fmt.Println(myAtoi("   -422"))
	fmt.Println(myAtoi("+42"))
}
