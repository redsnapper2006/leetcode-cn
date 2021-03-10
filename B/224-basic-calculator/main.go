package main

import "fmt"

func calculate(s string) int {
	nStack := []int{}
	oStack := []byte{}
	bStack := []int{}

	p := byte('#')
	for i, b := range s {
		if b == ' ' {
			continue
		}

		if b == '(' {
			bStack = append(bStack, len(nStack))
		} else if b == '+' || b == '-' {
			if p == '#' || p == '(' {
				nStack = append(nStack, 0)
			}
			oStack = append(oStack, byte(b))
		} else if b == ')' {
			stop := bStack[len(bStack)-1]
			bStack = bStack[0 : len(bStack)-1]
			n := nStack[stop]
			oIdx := len(oStack) - (len(nStack) - stop - 1)
			for i := stop + 1; i < len(nStack); i++ {
				ops := oStack[oIdx]
				oIdx++
				if ops == '+' {
					n += nStack[i]
				} else {
					n -= nStack[i]
				}
			}
			oStack = oStack[0 : len(oStack)-(len(nStack)-stop-1)]
			nStack = nStack[0 : stop+1]
			nStack[stop] = n
		} else {
			if i == 0 || s[i-1] > '9' || s[i-1] < '0' {
				nStack = append(nStack, int(byte(b)-'0'))
			} else {
				nStack[len(nStack)-1] = nStack[len(nStack)-1]*10 + int(byte(b)-'0')
			}
		}
		p = byte(b)
	}

	if len(nStack) > 1 {
		n := nStack[0]
		oIdx := 0
		for i := 1; i < len(nStack); i++ {
			ops := oStack[oIdx]
			oIdx++
			if ops == '+' {
				n += nStack[i]
			} else {
				n -= nStack[i]
			}
		}
		nStack[0] = n
	}

	return nStack[0]
}

func main() {
	fmt.Println("")
}
