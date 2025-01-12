package main

import "fmt"

func translateNum(num int) int {
	if num == 0 {
		return 1
	}
	var digits []int
	c := 0
	for num > 0 {
		c++
		digits = append(digits, num%10)
		num /= 10
	}
	buf := make([]int, c)
	buf[0] = 1
	for i := 1; i < c; i++ {
		p1 := buf[i-1]
		p2 := 0
		if i > 0 && (digits[i] == 1 || digits[i] == 2) {

			t := digits[i]*10 + digits[i-1]
			if t >= 10 && t <= 25 {
				p2 = 1
				if i > 1 {
					p2 = buf[i-2]
				}
			}
		}
		buf[i] = p1 + p2
	}
	return buf[c-1]
}

func main() {
	fmt.Println("a")
}
