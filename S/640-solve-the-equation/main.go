package main

import (
	"fmt"
	"strings"
)

func solveEquation(equation string) string {
	// x+5-3+x=6+x-2

	eqArr := strings.Split(equation, "=")

	format := func(eq string) (int, int) {
		isPositive := true
		multi := 0
		isX := false
		isSet := false
		xpart, cpart := 0, 0
		eq += "+"
		for i := 0; i < len(eq); i++ {
			if eq[i] == '-' || eq[i] == '+' {
				if !isPositive {
					multi = -multi
				}
				if isX {
					xpart += multi
				} else {
					cpart += multi
				}
				isPositive = eq[i] == '+'
				isX = false
				isSet = false
				multi = 0
			} else if eq[i] <= '9' && eq[i] >= '0' {
				multi = multi*10 + int(eq[i]-'0')
				isSet = true
			} else if eq[i] == 'x' {
				if !isSet {
					multi = 1
				}
				isX = true
			}
		}
		return xpart, cpart
	}
	leftx, leftc := format(eqArr[0])
	rightx, rightc := format(eqArr[1])

	x := leftx - rightx
	c := rightc - leftc
	if x == 0 {
		if c == 0 {
			return "Infinite solutions"
		} else {
			return "No solution"
		}
	}
	return fmt.Sprintf("x=%d", (c / x))
}

func main() {
	fmt.Println("a")
}
